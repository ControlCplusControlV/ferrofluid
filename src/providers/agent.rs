//! Agent wallet management with automatic rotation and safety features

use crate::{
    errors::HyperliquidError, providers::nonce::NonceManager, signers::HyperliquidSigner,
    Network,
};
use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Agent wallet with lifecycle tracking
#[derive(Clone)]
pub struct AgentWallet {
    /// Agent's address
    pub address: Address,
    /// Agent's signer
    pub signer: PrivateKeySigner,
    /// When this agent was created
    pub created_at: Instant,
    /// Dedicated nonce manager for this agent
    pub nonce_manager: Arc<NonceManager>,
    /// Current status
    pub status: AgentStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AgentStatus {
    /// Agent is active and healthy
    Active,
    /// Agent is marked for rotation
    PendingRotation,
    /// Agent has been deregistered
    Deregistered,
}

impl AgentWallet {
    /// Create a new agent wallet
    pub fn new(signer: PrivateKeySigner) -> Self {
        Self {
            address: signer.address(),
            signer,
            created_at: Instant::now(),
            nonce_manager: Arc::new(NonceManager::new(false)), // No isolation within agent
            status: AgentStatus::Active,
        }
    }

    /// Check if agent should be rotated based on TTL
    pub fn should_rotate(&self, ttl: Duration) -> bool {
        match self.status {
            AgentStatus::Active => self.created_at.elapsed() > ttl,
            AgentStatus::PendingRotation | AgentStatus::Deregistered => true,
        }
    }

    /// Get next nonce for this agent
    pub fn next_nonce(&self) -> u64 {
        self.nonce_manager.next_nonce(None)
    }
}

/// Configuration for agent management
#[derive(Clone, Debug)]
pub struct AgentConfig {
    /// Time before rotating an agent
    pub ttl: Duration,
    /// Check agent health at this interval
    pub health_check_interval: Duration,
    /// Rotate agents proactively before expiry
    pub proactive_rotation_buffer: Duration,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            ttl: Duration::from_secs(23 * 60 * 60), // Rotate daily
            health_check_interval: Duration::from_secs(300), // Check every 5 min
            proactive_rotation_buffer: Duration::from_secs(60 * 60), // Rotate 1hr before expiry
        }
    }
}

/// Manages agent lifecycle with automatic rotation
pub struct AgentManager<S: HyperliquidSigner> {
    /// Master signer that approves agents
    master_signer: S,
    /// Currently active agents by name
    agents: Arc<RwLock<std::collections::HashMap<String, AgentWallet>>>,
    /// Configuration
    config: AgentConfig,
    /// Network for agent operations
    network: Network,
}

impl<S: HyperliquidSigner + Clone> AgentManager<S> {
    /// Create a new agent manager
    pub fn new(master_signer: S, config: AgentConfig, network: Network) -> Self {
        Self {
            master_signer,
            agents: Arc::new(RwLock::new(std::collections::HashMap::new())),
            config,
            network,
        }
    }

    /// Get or create an agent, rotating if necessary
    pub async fn get_or_rotate_agent(
        &self,
        name: &str,
    ) -> Result<AgentWallet, HyperliquidError> {
        let mut agents = self.agents.write().await;

        // Check if we have an active agent
        if let Some(agent) = agents.get(name) {
            let effective_ttl = self
                .config
                .ttl
                .saturating_sub(self.config.proactive_rotation_buffer);

            if !agent.should_rotate(effective_ttl) {
                return Ok(agent.clone());
            }

            // Mark for rotation
            let mut agent_mut = agent.clone();
            agent_mut.status = AgentStatus::PendingRotation;
            agents.insert(name.to_string(), agent_mut);
        }

        // Create new agent
        let new_agent = self.create_new_agent(name).await?;
        agents.insert(name.to_string(), new_agent.clone());

        Ok(new_agent)
    }

    /// Create and approve a new agent
    async fn create_new_agent(
        &self,
        name: &str,
    ) -> Result<AgentWallet, HyperliquidError> {
        // Generate new key for agent
        let agent_signer = PrivateKeySigner::random();
        let agent_wallet = AgentWallet::new(agent_signer.clone());

        // We need to approve this agent using the exchange provider
        // This is a bit circular, but we'll handle it carefully
        self.approve_agent_internal(agent_wallet.address, Some(name.to_string()))
            .await?;

        Ok(agent_wallet)
    }

    /// Internal method to approve agent (will use exchange provider)
    async fn approve_agent_internal(
        &self,
        agent_address: Address,
        name: Option<String>,
    ) -> Result<(), HyperliquidError> {
        use crate::providers::RawExchangeProvider;

        // Create a temporary raw provider just for agent approval
        let raw_provider = match self.network {
            Network::Mainnet => RawExchangeProvider::mainnet(self.master_signer.clone()),
            Network::Testnet => RawExchangeProvider::testnet(self.master_signer.clone()),
        };

        // Approve the agent
        raw_provider.approve_agent(agent_address, name).await?;

        Ok(())
    }

    /// Get all active agents
    pub async fn get_active_agents(&self) -> Vec<(String, AgentWallet)> {
        let agents = self.agents.read().await;
        agents
            .iter()
            .filter(|(_, agent)| agent.status == AgentStatus::Active)
            .map(|(name, agent)| (name.clone(), agent.clone()))
            .collect()
    }

    /// Mark an agent as deregistered
    pub async fn mark_deregistered(&self, name: &str) {
        let mut agents = self.agents.write().await;
        if let Some(agent) = agents.get_mut(name) {
            agent.status = AgentStatus::Deregistered;
        }
    }

    /// Clean up deregistered agents
    pub async fn cleanup_deregistered(&self) {
        let mut agents = self.agents.write().await;
        agents.retain(|_, agent| agent.status != AgentStatus::Deregistered);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_rotation_check() {
        let signer = PrivateKeySigner::random();
        let agent = AgentWallet::new(signer);

        // Should not rotate immediately
        assert!(!agent.should_rotate(Duration::from_secs(24 * 60 * 60)));

        // Test with zero duration (should always rotate)
        assert!(agent.should_rotate(Duration::ZERO));
    }

    #[test]
    fn test_agent_nonce_generation() {
        let signer = PrivateKeySigner::random();
        let agent = AgentWallet::new(signer);

        let nonce1 = agent.next_nonce();
        let nonce2 = agent.next_nonce();

        assert!(nonce2 > nonce1);
    }
}
