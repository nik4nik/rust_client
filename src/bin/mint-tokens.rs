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
	commitment_config::CommitmentConfig
};
use spl_token::instruction::mint_to;

fn main() {
	// Load environment variables from .env file
	dotenv().ok(); // does not support multiline values!

    // Загружаем секретный ключ из .env
    let private_key = env::var("SECRET_KEY").expect("Добавьте SECRET_KEY в .env!");
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Невозможно декодировать ключ");
    let sender = Keypair::from_bytes(&as_array).expect("Невозможно создать Keypair из секретного ключа");

    // Подключаемся к Solana Devnet
    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    // Устанавливаем количество десятичных знаков (2)
    let minor_units_per_major_units = 10u64.pow(2);

    // Адрес токенового аккаунта (Mint)
    let token_mint_account = Pubkey::from_str("GLCkK1D5aKAaeeQSLRXHLzdWrrkmad2rJXBD3A5mWTis")
        .expect("Невозможно создать Pubkey из строки");

    // Адрес ассоциированного токен аккаунта получателя
    let recipient_associated_token_account = Pubkey::from_str("2LCfqLW1tRbDqQGpdYXugrfoEq2vVkNn8Vh9gT6Rd2gR")
        .expect("Невозможно создать Pubkey из строки");

    // Создаем инструкцию для Minting токенов
    let mint_to_ix = mint_to(
        &spl_token::id(),
        &token_mint_account,
        &recipient_associated_token_account,
        &sender.pubkey(),
        &[],
        10 * minor_units_per_major_units,
    ).expect("Ошибка при создании инструкции MintTo");

    // Создаем транзакцию
    let mut transaction = Transaction::new_with_payer(
        &[mint_to_ix],
        Some(&sender.pubkey()),
    );

    // Получаем последний блокхэш
    let recent_blockhash = connection
		.get_latest_blockhash()
		.expect("Ошибка при получении последнего блокхэша");
    transaction.sign(&[&sender], recent_blockhash);

    // Отправляем транзакцию и получаем ее сигнатуру
    let transaction_signature = connection
        .send_and_confirm_transaction(&transaction)
        .expect("Ошибка при отправке транзакции");

    // Получаем ссылку на Explorer
    println!("Успех! Транзакция Mint Token: {}", format!(
		"https://explorer.solana.com/address/{}?cluster=devnet",
		&transaction_signature.to_string()
	));
}