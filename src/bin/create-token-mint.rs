use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
	signature::{Keypair, Signer},
	program_pack::Pack,
	pubkey::Pubkey,
	system_instruction,
	transaction::Transaction,
};
use spl_token::{
	state::Mint,
	instruction::initialize_mint
};

const URL: &str = "https://api.devnet.solana.com";

fn main() {
	// Load environment variables from .env file
	dotenv().ok(); // does not support multiline values!

	// Загрузить секретный ключ из .env
	let private_key = env::var("SECRET_KEY").expect("Добавьте SECRET_KEY в .env!");
	let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Невозможно декодировать ключ");
	let sender = Keypair::from_bytes(&as_array).expect("Невозможно создать Keypair из секретного ключа");

	// Подключение к Solana Devnet
	let connection = RpcClient::new(URL);

	println!("Наш публичный ключ: {}", sender.pubkey());

	// Создать токен
	let mint = create_mint(
		&connection,
		&sender,
		&sender.pubkey(),
		None,
		2,
	).expect("Невозможно создать токен");

	// Получить ссылку на Explorer
	let link = format!("https://explorer.solana.com/address/{}?cluster=devnet", &mint.to_string());

	println!("Token Mint: {}", link);
}

// Создание токена
fn create_mint(
	connection: &RpcClient,
	payer: &Keypair,
	mint_authority: &Pubkey,
	freeze_authority: Option<&Pubkey>,
	decimals: u8,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
	let mint = Keypair::new();
	let lamports = connection.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

	let mut transaction = Transaction::new_with_payer(
		&[
			system_instruction::create_account(
				&payer.pubkey(),
				&mint.pubkey(),
				lamports,
				Mint::LEN as u64,
				&spl_token::id(),
			),
			initialize_mint(
				&spl_token::id(),
				&mint.pubkey(),
				mint_authority,
				freeze_authority,
				decimals,
			)?,
		],
		Some(&payer.pubkey()),
	);

	// Получаем последний блокхэш
	let recent_blockhash = connection
		.get_latest_blockhash()
		.expect("Ошибка при получении последнего блокхэша");
	transaction.sign(&[payer, &mint], recent_blockhash);

	let signature = connection.send_and_confirm_transaction(&transaction)?;
	println!("Transaction signature: {}", signature);

	Ok(mint.pubkey())
}

/* вывод
Наш публичный ключ: B4BTHaFFMzBe4Kbxdo7xxte6nw3jzvgyKPE3dPPy4JLc
Transaction signature: 5FymUgFwKeGENLQ7twG91DTvEPqu1K44cap17TYdDc4GZDYNA39Ety8BMs4WtdsTyUaKk1PY4utnZLeFepx4y8Zw
Token Mint: https://explorer.solana.com/address/2oei7zbtjamdw3Y3426MUgiPKQMmPmBpppp1wn8DZBU2?cluster=devnet
*/