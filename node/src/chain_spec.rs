use build3_node_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("rpc"),
		// Fork ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet Mod",
		// ID
		"local_testnet",
		ChainType::Local,
		move || -> GenesisConfig {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					authority_keys_from_seed("Kenny"),
					authority_keys_from_seed("Jayce"),
					authority_keys_from_seed("Simon"),
					authority_keys_from_seed("Phil"),
					authority_keys_from_seed("Colby"),
				],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Kenny"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Kenny"),
					get_account_id_from_seed::<sr25519::Public>("Jayce"),
					get_account_id_from_seed::<sr25519::Public>("Simon"),
					get_account_id_from_seed::<sr25519::Public>("Phil"),
					get_account_id_from_seed::<sr25519::Public>("Colby"),
					get_account_id_from_seed::<sr25519::Public>("Ricardo"),
					get_account_id_from_seed::<sr25519::Public>("Ben"),
					get_account_id_from_seed::<sr25519::Public>("Midhun"),
					get_account_id_from_seed::<sr25519::Public>("MV"),
					get_account_id_from_seed::<sr25519::Public>("Herin"),
					get_account_id_from_seed::<sr25519::Public>("Gokul"),
					get_account_id_from_seed::<sr25519::Public>("Binoy"),
					get_account_id_from_seed::<sr25519::Public>("Ramdas"),
					get_account_id_from_seed::<sr25519::Public>("Shifin"),
					get_account_id_from_seed::<sr25519::Public>("Akhil"),
					get_account_id_from_seed::<sr25519::Public>("Sreejith"),
					get_account_id_from_seed::<sr25519::Public>("Arun"),
					get_account_id_from_seed::<sr25519::Public>("Carter"),
					get_account_id_from_seed::<sr25519::Public>("Praveen"),
					get_account_id_from_seed::<sr25519::Public>("Rathika"),
					get_account_id_from_seed::<sr25519::Public>("Sidney"),
					get_account_id_from_seed::<sr25519::Public>("SteveK"),
					get_account_id_from_seed::<sr25519::Public>("Suraj"),
					get_account_id_from_seed::<sr25519::Public>("Kenny//stash"),
					get_account_id_from_seed::<sr25519::Public>("Jayce//stash"),
					get_account_id_from_seed::<sr25519::Public>("Simon//stash"),
					get_account_id_from_seed::<sr25519::Public>("Phil//stash"),
					get_account_id_from_seed::<sr25519::Public>("Colby//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ricardo//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ben//stash"),
					get_account_id_from_seed::<sr25519::Public>("Midhun//stash"),
					get_account_id_from_seed::<sr25519::Public>("MV//stash"),
					get_account_id_from_seed::<sr25519::Public>("Herin//stash"),
					get_account_id_from_seed::<sr25519::Public>("Gokul//stash"),
					get_account_id_from_seed::<sr25519::Public>("Binoy//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ramdas//stash"),
					get_account_id_from_seed::<sr25519::Public>("Shifin//stash"),
					get_account_id_from_seed::<sr25519::Public>("Akhil//stash"),
					get_account_id_from_seed::<sr25519::Public>("Sreejith//stash"),
					get_account_id_from_seed::<sr25519::Public>("Arun//stash"),
					get_account_id_from_seed::<sr25519::Public>("Carter//stash"),
					get_account_id_from_seed::<sr25519::Public>("Praveen//stash"),
					get_account_id_from_seed::<sr25519::Public>("Rathika//stash"),
					get_account_id_from_seed::<sr25519::Public>("Sidney//stash"),
					get_account_id_from_seed::<sr25519::Public>("Suraj//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("rpc"),
		// Fork ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
