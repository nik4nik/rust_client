use dotenv::dotenv;
use std::{
	env,
	str::FromStr
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
	signature::{Keypair, Signer},
	pubkey::Pubkey,
	transaction::Transaction,
};
use spl_associated_token_account::{
	get_associated_token_address,
	instruction::create_associated_token_account
};
use spl_token::ID as TOKEN_PROGRAM_ID;

const URL: &str = "https://api.devnet.solana.com";

fn main() {
	// Load environment variables from .env file
	dotenv().ok(); // does not support multiline values!

	// Загружаем секретный ключ из .env
	let private_key = env::var("SECRET_KEY").expect("Добавьте SECRET_KEY в .env!");
	let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Невозможно декодировать ключ");
	let sender = Keypair::from_bytes(&as_array).expect("Невозможно создать Keypair из секретного ключа");

	// Подключаемся к Solana Devnet
	let connection = RpcClient::new(URL);

	println!("Наш публичный ключ: {}", sender.pubkey());

	// Определяем аккаунт токена
	let token_mint_account = Pubkey::from_str("GLCkK1D5aKAaeeQSLRXHLzdWrrkmad2rJXBD3A5mWTis")
		.expect("Невозможно создать Pubkey из строки");
	let recipient = Pubkey::from_str("3xwt5cT4Sc1XfJ8gv8nycaY4gz3S2XqJCB5VpbyXW2DY")
		.expect("Невозможно создать Pubkey из строки");

	// Получаем или создаем ассоциированный токен аккаунт
	let token_account = get_or_create_associated_token_account(
		&connection,
		&sender,
		&token_mint_account,
		&recipient,
	).expect("Не удалось получить или создать ассоциированный токен аккаунт");

	println!("Token Account: {}", token_account.to_string());

	// Получаем ссылку на Explorer
	println!("Созданный токен аккаунт: {}", format!(
		"https://explorer.solana.com/address/{}?cluster=devnet",
		&token_account.to_string()
	));
}

// Функция для получения или создания ассоциированного токен аккаунта
fn get_or_create_associated_token_account(
	connection: &RpcClient,
	payer: &Keypair,
	mint: &Pubkey,
	owner: &Pubkey,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
	let associated_token_address = get_associated_token_address(owner, mint);

	if let Ok(_account) = connection.get_account(&associated_token_address) {
		// Если аккаунт уже существует, возвращаем его
		println!("Associated token account already exists: {associated_token_address}");
		Ok(associated_token_address)
	} else {
		// Если аккаунт не существует, создаем его
		let create_account_ix = create_associated_token_account(
			&payer.pubkey(), // Funding address
			owner,			 // Wallet address
			mint,			 // Token mint address
			&TOKEN_PROGRAM_ID, // Token program ID
		);

		let mut transaction = Transaction::new_with_payer(
			&[create_account_ix],
			Some(&payer.pubkey()),
		);

		// Получаем последний блокхэш
		let recent_blockhash = connection
			.get_latest_blockhash()
			.expect("Ошибка при получении последнего блокхэша");
		transaction.sign(&[payer], recent_blockhash);


		connection.send_and_confirm_transaction(&transaction)?;
		Ok(associated_token_address)
	}
}
/*
Наш публичный ключ: B4BTHaFFMzBe4Kbxdo7xxte6nw3jzvgyKPE3dPPy4JLc
Associated token account already exists: 2LCfqLW1tRbDqQGpdYXugrfoEq2vVkNn8Vh9gT6Rd2gR
Token Account: 2LCfqLW1tRbDqQGpdYXugrfoEq2vVkNn8Vh9gT6Rd2gR
Созданный токен аккаунт: https://explorer.solana.com/address/2LCfqLW1tRbDqQGpdYXugrfoEq2vVkNn8Vh9gT6Rd2gR?cluster=devnet
*/