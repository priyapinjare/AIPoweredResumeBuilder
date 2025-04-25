#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, symbol_short, Symbol, log};

#[contracttype]
#[derive(Clone)]
pub struct Resume {
    pub id: u64,
    pub user_name: String,
    pub summary: String,
    pub created_at: u64,
    pub is_active: bool,
}

#[contracttype]
pub enum ResumeKey {
    ById(u64),
}

const RESUME_COUNTER: Symbol = symbol_short!("R_COUNT");

#[contract]
pub struct ResumeBuilderContract;

#[contractimpl]
impl ResumeBuilderContract {
    pub fn create_resume(env: Env, user_name: String, summary: String) -> u64 {
        let mut resume_count: u64 = env.storage().instance().get(&RESUME_COUNTER).unwrap_or(0);
        resume_count += 1;

        let timestamp = env.ledger().timestamp();

        let resume = Resume {
            id: resume_count,
            user_name,
            summary,
            created_at: timestamp,
            is_active: true,
        };

        env.storage().instance().set(&ResumeKey::ById(resume.id), &resume);
        env.storage().instance().set(&RESUME_COUNTER, &resume_count);
        log!(&env, "Resume Created: ID {}", resume.id);

        resume.id
    }

    pub fn deactivate_resume(env: Env, id: u64) {
        let mut resume: Resume = env
            .storage()
            .instance()
            .get(&ResumeKey::ById(id))
            .unwrap_or_else(|| panic!("Resume not found"));

        resume.is_active = false;
        env.storage().instance().set(&ResumeKey::ById(id), &resume);
        log!(&env, "Resume Deactivated: ID {}", id);
    }

    pub fn view_resume(env: Env, id: u64) -> Resume {
        env.storage()
            .instance()
            .get(&ResumeKey::ById(id))
            .unwrap_or_else(|| panic!("Resume not found"))
    }
}