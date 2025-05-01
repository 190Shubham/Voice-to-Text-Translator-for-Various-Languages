
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct TranslationRecord {
    pub id: u64,
    pub input_language: String,
    pub output_language: String,
    pub original_text: String,
    pub translated_text: String,
}

#[contracttype]
pub enum TranslationKey {
    Record(u64),
}

const RECORD_COUNT: Symbol = symbol_short!("COUNT");

#[contract]
pub struct VoiceToTextContract;

#[contractimpl]
impl VoiceToTextContract {
    pub fn submit_translation(
        env: Env,
        input_lang: String,
        output_lang: String,
        original: String,
        translated: String,
    ) -> u64 {
        let mut count: u64 = env.storage().instance().get(&RECORD_COUNT).unwrap_or(0);
        count += 1;

        let record = TranslationRecord {
            id: count,
            input_language: input_lang,
            output_language: output_lang,
            original_text: original,
            translated_text: translated,
        };

        env.storage().instance().set(&TranslationKey::Record(count), &record);
        env.storage().instance().set(&RECORD_COUNT, &count);

        count
    }

    pub fn get_translation(env: Env, id: u64) -> TranslationRecord {
        env.storage().instance().get(&TranslationKey::Record(id)).unwrap_or(TranslationRecord {
            id: 0,
            input_language: String::from_str(&env, "N/A"),
            output_language: String::from_str(&env, "N/A"),
            original_text: String::from_str(&env, "Not Found"),
            translated_text: String::from_str(&env, "Not Found"),
        })
    }
}

