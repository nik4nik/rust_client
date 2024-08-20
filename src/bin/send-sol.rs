use dotenv::dotenv;
use std::{
	env,
	str::FromStr
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
	signature::{Keypair, Signer},
	transaction::Transaction,
	pubkey::Pubkey,
	system_instruction,
	instruction::Instruction
};

const URL: &str = "https://api.devnet.solana.com";

fn main() {
	// Load environment variables from .env file
	dotenv().ok(); // does not support multiline values!

	// Загружаем секретный ключ из .env
	let private_key = env::var("SECRET_KEY").expect("Добавьте SECRET_KEY в .env!");

	// Use the SECRET_KEY in your application
	println!("Loaded secret key: {}", private_key);

	let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Невозможно декодировать ключ");
	let sender = Keypair::from_bytes(&as_array).expect("Невозможно создать Keypair из секретного ключа");

	// Подключаемся к Solana Devnet
	let connection = RpcClient::new(URL);

	println!("Наш публичный ключ: {}", sender.pubkey());

	// Адрес получателя
	let recipient = Pubkey::from_str("3xwt5cT4Sc1XfJ8gv8nycaY4gz3S2XqJCB5VpbyXW2DY")
		.expect("Невозможно создать Pubkey из строки");
	println!("Попытка отправить 0.01 SOL на {}", recipient);

   // Адрес программы Memo
	let memo_program = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")
		.expect("Невозможно создать Pubkey из строки");

	let memo_text = "and if I had seen as many kings or archbishops I could not have been more delighted";
	println!("Memo: {}", memo_text);

	let memo_instruction = Instruction {
		program_id: memo_program,
		accounts: vec![],
		data: memo_text.as_bytes().to_vec(),
	};

	let transfer_amount = 0.01;

	let mut transaction = Transaction::new_with_payer(
		&[
			// Инструкция для перевода SOL
			system_instruction::transfer(
				&sender.pubkey(),
				&recipient,
				(transfer_amount * solana_sdk::native_token::LAMPORTS_PER_SOL as f64) as u64,
			),
			memo_instruction,
		],
		Some(&sender.pubkey()),
	);

	// Получаем последний блокхэш
	let recent_blockhash = connection
		.get_latest_blockhash()
		.expect("Ошибка при получении последнего блокхэша");
	transaction.sign(&[&sender], recent_blockhash);

	// Отправляем и подтверждаем транзакцию
	let signature = connection
		.send_and_confirm_transaction(&transaction)
		.expect("Ошибка при отправке транзакции");

	println!("Транзакция подтверждена, сигнатура: {}!", signature);
}