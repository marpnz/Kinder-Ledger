use anchor_lang::prelude::*;

// NOTA: Al hacer 'Build' en Playground, se generará un ID automáticamente. 
// Copia ese ID y reemplázalo aquí si es necesario.
declare_id!("6k7...tu_program_id_aqui..."); 

#[program]
pub mod kindred_ledger {
    use super::*;

    /// Registra a un niño de forma permanente en la blockchain.
    pub fn register_child(
        ctx: Context<RegisterChild>, 
        id_expediente: String, 
        name: String, 
        age: u8,
        medical_notes: String
    ) -> Result<()> {
        let child_record = &mut ctx.accounts.child_record;
        let clock = Clock::get()?; // Captura el tiempo real de la red Solana

        child_record.admin = *ctx.accounts.admin.key;
        child_record.id_expediente = id_expediente;
        child_record.name = name;
        child_record.age = age;
        child_record.medical_notes = medical_notes;
        child_record.admission_date = clock.unix_timestamp;
        child_record.is_active = true;

        msg!("Expediente creado permanentemente: {}", child_record.id_expediente);
        Ok(())
    }

    /// Actualiza información sin borrar el registro original.
    /// Ideal para cuando un niño egresa o cambia su situación médica.
    pub fn update_child_status(
        ctx: Context<UpdateChild>, 
        new_notes: String, 
        is_active: bool
    ) -> Result<()> {
        let child_record = &mut ctx.accounts.child_record;
        
        child_record.medical_notes = new_notes;
        child_record.is_active = is_active;

        msg!("Registro actualizado para el expediente: {}", child_record.id_expediente);
        Ok(())
    }
}

#[account]
pub struct ChildRecord {
    pub admin: Pubkey,          // 32 bytes
    pub id_expediente: String,  // 4 + 16 bytes (ej. "CASA-001")
    pub name: String,           // 4 + 32 bytes
    pub medical_notes: String,  // 4 + 128 bytes
    pub age: u8,                // 1 byte
    pub admission_date: i64,    // 8 bytes (timestamp)
    pub is_active: bool,        // 1 byte
}

#[derive(Accounts)]
#[instruction(id_expediente: String)]
pub struct RegisterChild<'info> {
    #[account(
        init, 
        payer = admin, 
        // Cálculo de espacio: Discriminante (8) + Campos definidos arriba
        space = 8 + 32 + 20 + 36 + 132 + 1 + 8 + 1,
        // La PDA se genera usando la palabra "child" y el ID del expediente
        seeds = [b"child", id_expediente.as_bytes()],
        bump
    )]
    pub child_record: Account<'info, ChildRecord>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateChild<'info> {
    #[account(
        mut, 
        // Solo el administrador que creó el registro puede editarlo
        has_one = admin @ ErrorCode::Unauthorized
    )]
    pub child_record: Account<'info, ChildRecord>,
    pub admin: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No tienes permiso para modificar este expediente.")]
    Unauthorized,
}
