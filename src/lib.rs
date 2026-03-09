use anchor_lang::prelude::*;

declare_id!("6k7...tu_program_id_aqui..."); 

#[program]
pub mod kindred_ledger {
    use super::*;

    /// 1. Registro inicial del niño
    pub fn register_child(
        ctx: Context<RegisterChild>, 
        id_expediente: String, 
        name: String, 
        age: u8,
        blood_type: String,
        initial_notes: String
    ) -> Result<()> {
        let child_record = &mut ctx.accounts.child_record;
        let clock = Clock::get()?; 

        child_record.admin = *ctx.accounts.admin.key;
        child_record.id_expediente = id_expediente;
        child_record.name = name;
        child_record.age = age;
        child_record.blood_type = blood_type;
        child_record.medical_notes = initial_notes;
        child_record.admission_date = clock.unix_timestamp;
        child_record.is_active = true;
        child_record.exit_date = 0; // 0 significa que sigue en la casa

        msg!("Expediente permanente creado: {}", child_record.name);
        Ok(())
    }

    /// 2. Actualizar edad y salud (Crecimiento del niño)
    pub fn update_vitals(ctx: Context<UpdateChild>, new_age: u8, new_notes: String) -> Result<()> {
        let child_record = &mut ctx.accounts.child_record;
        
        child_record.age = new_age;
        child_record.medical_notes = new_notes;

        msg!("Vitales actualizados para: {}", child_record.id_expediente);
        Ok(())
    }

    /// 3. Registrar Salida (Egreso) - El registro queda inactivo pero LA DATA SIGUE AHÍ
    pub fn register_exit(ctx: Context<UpdateChild>, exit_reason: String) -> Result<()> {
        let child_record = &mut ctx.accounts.child_record;
        let clock = Clock::get()?;

        child_record.is_active = false;
        child_record.exit_date = clock.unix_timestamp;
        // Concatenamos la razón de salida a las notas médicas para que quede el registro
        child_record.medical_notes = format!("{} | MOTIVO SALIDA: {}", child_record.medical_notes, exit_reason);

        msg!("Egreso registrado. El expediente ahora es histórico.");
        Ok(())
    }
}

#[account]
pub struct ChildRecord {
    pub admin: Pubkey,          // 32 bytes
    pub id_expediente: String,  // 4 + 20 bytes
    pub name: String,           // 4 + 40 bytes
    pub blood_type: String,     // 4 + 4 bytes (ej. "O+", "AB-")
    pub medical_notes: String,  // 4 + 200 bytes (más espacio para historial)
    pub age: u8,                // 1 byte
    pub admission_date: i64,    // 8 bytes
    pub exit_date: i64,         // 8 bytes
    pub is_active: bool,        // 1 byte
}

#[derive(Accounts)]
#[instruction(id_expediente: String)]
pub struct RegisterChild<'info> {
    #[account(
        init, 
        payer = admin, 
        // Aumentamos el espacio para notas más largas y nuevos campos
        space = 8 + 32 + 24 + 44 + 8 + 204 + 1 + 8 + 8 + 1,
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
        has_one = admin @ ErrorCode::UnauthorizedAccess,
        constraint = child_record.is_active == true @ ErrorCode::RecordAlreadyInactive
    )]
    pub child_record: Account<'info, ChildRecord>,
    pub admin: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No tienes permisos de administrador sobre este expediente.")]
    UnauthorizedAccess,
    #[msg("Este expediente ya está cerrado (el niño ya egresó).")]
    RecordAlreadyInactive,
}
