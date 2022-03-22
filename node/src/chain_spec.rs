use build3_node_runtime::{
	AccountId, AuraConfig, BalancesConfig, BoardVirginiaConfig, CouncilConfig, GenesisConfig,
	GrandpaConfig, Signature, SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use sc_network::config::MultiaddrWithPeerId;

use hex_literal::hex;

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "ws://127.0.0.1:9944/";

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

/// Generate an Aura and GRANDPA authority key for a given seed.
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
			dev_genesis(
				wasm_binary,
				// Initial PoA authorities
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![authority_keys_from_seed("Alice")],
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					// Stash 1: 5Gmdzhhd6KavPysdrnMbywimYcUKtRLdEDQtbmJbdi7ZC5sU
					hex!["d028ff189adb5d633dc67f6d741acb8cbd42001dec5fc307c485585c22aefa2a"].into(),
					// Controller 1: 5HpnvpCwwbf3tYhM34cALYp2UnDZcvGvWSxpgq8v5zDHDQcB
					hex!["fecd09851c570b330866244c5f4af5f46d2ca1b2e91e464145ebc31aed069d61"].into(),
					// VA PE 1: 5Est8NkVq5aqsHPieNq89e24nCf6gUUKPhi7G66ytQRn5aF5
					hex!["7c739356df4ae8218719d0e6e6927fda4feb4dd8fcdc9271afbe805b8f276555"].into(),
					// NC PE 1: 5FWb5jWUZafgTzn4m7hXBwBPwxUb7q9zreyELGuJGRHuMYfJ
					hex!["9872466a3198ca024d8b9b4b7f883eaaf390fa4b465f04a2c944c9a93ffdac42"].into(),
					// Stash 2: 5DGBnLmbDpZhc85L9yrUDzRSqajZmcUWdNmsK2KmDfS4APjF
					hex!["34fdf02b416dd6b0287adc5d8a5444ac7c8ab0e1265f0681d1924d75e4b5876d"].into(),
					// Controller 2: 5GRSjNELxVXy6uo7paVrLwjEK2NwEDzT5tky4tgVGKCdqLQQ
					hex!["c0c22b43c2127b3a697aaf7617499a82beaf297e07ef56fba84ffd92207a7841"].into(),
					// VA PE 2: 5CqTHp7ykJLHmBFxNRFjwn3XqmvmizbqybvFnQhoeAXd3jp5
					hex!["2221021107fbe5de0445925098dfb6e825542125408ceaf7e64ba14e19d21450"].into(),
					// NC PE 2: 5D28AgYmqW9j3ExR9spDY5b1CyXCCxGYubnP2EL1c3KYU5Bh
					hex!["2a44557ea43b903114b210a63dce16f6023a408f8fcbbf18882605c26d76336a"].into(),
					// Stash 3: 5CQLaUBifLeqayc4WW1gdt1NrtjCVZLGyHeARoZGYevzyhF3
					hex!["0ef9468cec5329415d2ee4d9d59324e24def4f3c9df83aaf615a7bc5a1254d08"].into(),
					// Controller 3: 5ERJ2uNxW7qzWb8jBTkSXmo6rw3VwXSJuuqoZAR8Q3LUL1ZR
					hex!["682c6c59747c165003e9ca3e44bda8475cb8827c313ccb8bdc1dae8cd6824953"].into(),
					// VA PE 3: 5G4ACtpvTzDB6P9tdCyN48AhH2rzg9Vc4jiWUnA6ZMqqdnmh
					hex!["b08662d74abb9af331b6c805aab339a3bf9388b39079f8d553e4adcb3846ba07"].into(),
					// NC PE 3: 5FtFcTKnkhbcxiToXrQNsxv8RgLfuTudf9t5EkGErmLEsTB3
					hex!["a8f8218852c5f49ecd1c4216f2ef55cf42316af3ac6490653ea236718b5ba666"].into(),
				],
				// Council Accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					// Controller 1: 5HpnvpCwwbf3tYhM34cALYp2UnDZcvGvWSxpgq8v5zDHDQcB
					hex!["fecd09851c570b330866244c5f4af5f46d2ca1b2e91e464145ebc31aed069d61"].into(),
					// Controller 2: 5GRSjNELxVXy6uo7paVrLwjEK2NwEDzT5tky4tgVGKCdqLQQ
					hex!["c0c22b43c2127b3a697aaf7617499a82beaf297e07ef56fba84ffd92207a7841"].into(),
					// Controller 3: 5ERJ2uNxW7qzWb8jBTkSXmo6rw3VwXSJuuqoZAR8Q3LUL1ZR
					hex!["682c6c59747c165003e9ca3e44bda8475cb8827c313ccb8bdc1dae8cd6824953"].into(),
				],
				// VA Board Accounts
				vec![
					// Controller 1: 5HpnvpCwwbf3tYhM34cALYp2UnDZcvGvWSxpgq8v5zDHDQcB
					hex!["fecd09851c570b330866244c5f4af5f46d2ca1b2e91e464145ebc31aed069d61"].into(),
					// Controller 2: 5GRSjNELxVXy6uo7paVrLwjEK2NwEDzT5tky4tgVGKCdqLQQ
					hex!["c0c22b43c2127b3a697aaf7617499a82beaf297e07ef56fba84ffd92207a7841"].into(),
					// Controller 3: 5ERJ2uNxW7qzWb8jBTkSXmo6rw3VwXSJuuqoZAR8Q3LUL1ZR
					hex!["682c6c59747c165003e9ca3e44bda8475cb8827c313ccb8bdc1dae8cd6824953"].into(),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
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
	let initial_authorities: Vec<(AccountId, AccountId, AccountId, AccountId, AuraId, GrandpaId)> = vec![
		(
			// Stash 1: 5Gmdzhhd6KavPysdrnMbywimYcUKtRLdEDQtbmJbdi7ZC5sU
			hex!["d028ff189adb5d633dc67f6d741acb8cbd42001dec5fc307c485585c22aefa2a"].into(),
			// Controller 1: 5HpnvpCwwbf3tYhM34cALYp2UnDZcvGvWSxpgq8v5zDHDQcB
			hex!["fecd09851c570b330866244c5f4af5f46d2ca1b2e91e464145ebc31aed069d61"].into(),
			// VA PE 1: 5Est8NkVq5aqsHPieNq89e24nCf6gUUKPhi7G66ytQRn5aF5
			hex!["7c739356df4ae8218719d0e6e6927fda4feb4dd8fcdc9271afbe805b8f276555"].into(),
			// NC PE 1: 5FWb5jWUZafgTzn4m7hXBwBPwxUb7q9zreyELGuJGRHuMYfJ
			hex!["9872466a3198ca024d8b9b4b7f883eaaf390fa4b465f04a2c944c9a93ffdac42"].into(),
			// 5CnjKJhf7fR369DPwzdqsDbmAgK7t2jWLhE6rRvvdzj9jmQY
			hex!["200d382411810653ce03a307f3989c42c80b957785c60644ebdefa4b63561637"]
				.unchecked_into(),
			// 5E7ksqmrR5C3H83Gvp7zmtJXcD4gzKNuH2SXnbuHXFKiSdHZ
			hex!["5acc5ebecc94862176156cb7a60e29543fde22ccf535f83293a66ac979fad17d"]
				.unchecked_into(),
		),
		(
			// Stash 2: 5DGBnLmbDpZhc85L9yrUDzRSqajZmcUWdNmsK2KmDfS4APjF
			hex!["34fdf02b416dd6b0287adc5d8a5444ac7c8ab0e1265f0681d1924d75e4b5876d"].into(),
			// Controller 2: 5GRSjNELxVXy6uo7paVrLwjEK2NwEDzT5tky4tgVGKCdqLQQ
			hex!["c0c22b43c2127b3a697aaf7617499a82beaf297e07ef56fba84ffd92207a7841"].into(),
			// VA PE 2: 5CqTHp7ykJLHmBFxNRFjwn3XqmvmizbqybvFnQhoeAXd3jp5
			hex!["2221021107fbe5de0445925098dfb6e825542125408ceaf7e64ba14e19d21450"].into(),
			// NC PE 2: 5D28AgYmqW9j3ExR9spDY5b1CyXCCxGYubnP2EL1c3KYU5Bh
			hex!["2a44557ea43b903114b210a63dce16f6023a408f8fcbbf18882605c26d76336a"].into(),
			// 5He7XG6KpHCNcnzvUwsLTw6eKeewYCLfjF6aG2UzVdQgB6jf
			hex!["f6a7ee2e3504e9905ba902b2a08f7460ce2145bcda5f7e490c366f8918c0b0d5"]
				.unchecked_into(),
			// 5FTrgH2GXoeQ5LPxvgtyLrLBWW5HgGcP2P7hdahJwnWR3NTY
			hex!["965d09c967f3ecf4d0407049400f9dfd880fbc55a815fbf36886490d2385393a"]
				.unchecked_into(),
		),
		(
			// Stash 3: 5CQLaUBifLeqayc4WW1gdt1NrtjCVZLGyHeARoZGYevzyhF3
			hex!["0ef9468cec5329415d2ee4d9d59324e24def4f3c9df83aaf615a7bc5a1254d08"].into(),
			// Controller 3: 5ERJ2uNxW7qzWb8jBTkSXmo6rw3VwXSJuuqoZAR8Q3LUL1ZR
			hex!["682c6c59747c165003e9ca3e44bda8475cb8827c313ccb8bdc1dae8cd6824953"].into(),
			// VA PE 3: 5G4ACtpvTzDB6P9tdCyN48AhH2rzg9Vc4jiWUnA6ZMqqdnmh
			hex!["b08662d74abb9af331b6c805aab339a3bf9388b39079f8d553e4adcb3846ba07"].into(),
			// NC PE 3: 5FtFcTKnkhbcxiToXrQNsxv8RgLfuTudf9t5EkGErmLEsTB3
			hex!["a8f8218852c5f49ecd1c4216f2ef55cf42316af3ac6490653ea236718b5ba666"].into(),
			// 5ENiYqjtU4sNMv4CXBkESUXNssJdd3nsieY8uLoMUbGCeqrg
			hex!["6635373d257b5ddb8f198ba58b5ac2336d5d4801d2154d1895c2186be5fbbd60"]
				.unchecked_into(),
			// 5H5pZqFUuaa2R1nVQWJvxXdsMQj5wj8R8swcAEYhScjggg8W
			hex!["de06ffe68b3a8c048c489b5b792828385579d2c2c6105ff294268de5129afa0d"]
				.unchecked_into(),
		),
	];
	let boot_node: MultiaddrWithPeerId =
		"/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWNXgjVXBfDwc1RQ172VzMwD9CiaY5A6XTHhnkqXFu9buC"
			.parse()
			.unwrap();
	assert_eq!(
		boot_node.peer_id.to_base58(),
		"12D3KooWNXgjVXBfDwc1RQ172VzMwD9CiaY5A6XTHhnkqXFu9buC"
	);
	assert_eq!(boot_node.multiaddr.to_string(), "/ip4/127.0.0.1/tcp/30333");

	Ok(ChainSpec::from_genesis(
		// Name
		"Build3 Staging Testnet",
		// ID
		"build3_staging_testnet",
		ChainType::Local,
		move || {
			staging_genesis(
				wasm_binary,
				// Sudo account
				// SUDO DEV: 5CppV3WNkSy921u6AdBH9pG9qf2AFFwmKo4FgFH4d6ncXhFP
				hex!["21a51b57515eefd5c0b12bf02038f7e9faed69874f578c518ec45b23e58b34f4"].into(),
				// Initial PoA authorities
				initial_authorities.clone(),
				// Pre-funded accounts
				vec![
					// Stash 1: 5Gmdzhhd6KavPysdrnMbywimYcUKtRLdEDQtbmJbdi7ZC5sU
					hex!["d028ff189adb5d633dc67f6d741acb8cbd42001dec5fc307c485585c22aefa2a"].into(),
					// Controller 1: 5HpnvpCwwbf3tYhM34cALYp2UnDZcvGvWSxpgq8v5zDHDQcB
					hex!["fecd09851c570b330866244c5f4af5f46d2ca1b2e91e464145ebc31aed069d61"].into(),
					// Stash 2: 5DGBnLmbDpZhc85L9yrUDzRSqajZmcUWdNmsK2KmDfS4APjF
					hex!["34fdf02b416dd6b0287adc5d8a5444ac7c8ab0e1265f0681d1924d75e4b5876d"].into(),
					// Controller 2: 5GRSjNELxVXy6uo7paVrLwjEK2NwEDzT5tky4tgVGKCdqLQQ
					hex!["c0c22b43c2127b3a697aaf7617499a82beaf297e07ef56fba84ffd92207a7841"].into(),
					// Stash 3: 5CQLaUBifLeqayc4WW1gdt1NrtjCVZLGyHeARoZGYevzyhF3
					hex!["0ef9468cec5329415d2ee4d9d59324e24def4f3c9df83aaf615a7bc5a1254d08"].into(),
					// Controller 3: 5ERJ2uNxW7qzWb8jBTkSXmo6rw3VwXSJuuqoZAR8Q3LUL1ZR
					hex!["682c6c59747c165003e9ca3e44bda8475cb8827c313ccb8bdc1dae8cd6824953"].into(),
				],
				// Council Accounts
				vec![
					// Controller 1: 5HpnvpCwwbf3tYhM34cALYp2UnDZcvGvWSxpgq8v5zDHDQcB
					hex!["fecd09851c570b330866244c5f4af5f46d2ca1b2e91e464145ebc31aed069d61"].into(),
					// Controller 2: 5GRSjNELxVXy6uo7paVrLwjEK2NwEDzT5tky4tgVGKCdqLQQ
					hex!["c0c22b43c2127b3a697aaf7617499a82beaf297e07ef56fba84ffd92207a7841"].into(),
					// Controller 3: 5ERJ2uNxW7qzWb8jBTkSXmo6rw3VwXSJuuqoZAR8Q3LUL1ZR
					hex!["682c6c59747c165003e9ca3e44bda8475cb8827c313ccb8bdc1dae8cd6824953"].into(),
				],
				// VA Board Accounts
				vec![
					// Controller 1: 5HpnvpCwwbf3tYhM34cALYp2UnDZcvGvWSxpgq8v5zDHDQcB
					hex!["fecd09851c570b330866244c5f4af5f46d2ca1b2e91e464145ebc31aed069d61"].into(),
					// Controller 2: 5GRSjNELxVXy6uo7paVrLwjEK2NwEDzT5tky4tgVGKCdqLQQ
					hex!["c0c22b43c2127b3a697aaf7617499a82beaf297e07ef56fba84ffd92207a7841"].into(),
					// Controller 3: 5ERJ2uNxW7qzWb8jBTkSXmo6rw3VwXSJuuqoZAR8Q3LUL1ZR
					hex!["682c6c59747c165003e9ca3e44bda8475cb8827c313ccb8bdc1dae8cd6824953"].into(),
				],
				true,
			)
		},
		// Bootnodes
		vec![boot_node],
		// Telemetry
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		// Protocol ID
		Some("build3-local-staging-testnet"),
		// Fork ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Genesis Configurations
/// Configurations for dev and testnet.

/// Staging TestNet
fn staging_genesis(
	wasm_binary: &[u8],
	root_key: AccountId,
	initial_authorities: Vec<(AccountId, AccountId, AccountId, AccountId, AuraId, GrandpaId)>,
	endowed_accounts: Vec<AccountId>,
	council_accounts: Vec<AccountId>,
	board_va_accounts: Vec<AccountId>,
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
			authorities: initial_authorities.iter().map(|x| (x.4.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.5.clone(), 1)).collect(),
		},
		council: CouncilConfig {
			phantom: Default::default(),
			members: council_accounts.iter().cloned().collect(),
		},
		council_member_manager: Default::default(),
		board_virginia: BoardVirginiaConfig {
			phantom: Default::default(),
			members: board_va_accounts.iter().cloned().collect(),
		},
		board_virginia_member_manager: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}

/// Dev mode gensis configuration
fn dev_genesis(
	wasm_binary: &[u8],
	root_key: AccountId,
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	endowed_accounts: Vec<AccountId>,
	council_accounts: Vec<AccountId>,
	board_va_accounts: Vec<AccountId>,
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
		council: CouncilConfig {
			phantom: Default::default(),
			members: council_accounts.iter().cloned().collect(),
		},
		council_member_manager: Default::default(),
		board_virginia: BoardVirginiaConfig {
			phantom: Default::default(),
			members: board_va_accounts.iter().cloned().collect(),
		},
		board_virginia_member_manager: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
