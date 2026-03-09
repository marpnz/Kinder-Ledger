use anchor_lang::prelude::*;

declare_id!("hpGG7v5Y5XFtT96thymVC9sAxUNUGPHygjuzgoTcFp3");

#[program]
pub mod lumen {
    use super::*;

    /// Crea un nuevo producto en la tienda virtual.
    /// Cada producto se almacena en una PDA única basada en:
    ///   - "item_v1"
    ///   - owner_pubkey
    ///   - título del producto
    pub fn add_item(ctx: Context<AddItem>, title: String, price: u64) -> Result<()> {
        // Validaciones de entrada
        require!(title.len() > 0 && title.len() <= 32, LumenError::InvalidTitleLength);
        require!(price > 0, LumenError::InvalidPrice);

        let item = &mut ctx.accounts.store_item;

        item.owner = *ctx.accounts.owner.key;
        item.title = title;
        item.price = price;
        item.is_available = true;
        item.bump = ctx.bumps.store_item;

        msg!("LUMEN: Producto '{}' registrado exitosamente.", item.title);
        Ok(())
    }

    /// Actualiza el precio y la disponibilidad de un producto.
    /// El sistema valida automáticamente que el `owner` sea el dueño real.
    pub fn update_item(
        ctx: Context<UpdateItem>,
        price: u64,
        is_available: bool,
    ) -> Result<()> {
        require!(price > 0, LumenError::InvalidPrice);

        let item = &mut ctx.accounts.store_item;

        item.price = price;
        item.is_available = is_available;

        msg!("LUMEN: Producto '{}' actualizado correctamente.", item.title);
        Ok(())
    }

    /// Elimina un producto y devuelve la renta (SOL) al propietario.
    pub fn delete_item(ctx: Context<DeleteItem>) -> Result<()> {
        let item = &ctx.accounts.store_item;

        msg!(
            "LUMEN: Producto '{}' eliminado. Renta devuelta al propietario {}.",
            item.title,
            item.owner
        );

        Ok(())
    }
}

/* -------------------------------------------------------------------------- */
/*                               MODELO DE DATOS                              */
/* -------------------------------------------------------------------------- */

#[account]
pub struct StoreItem {
    pub owner: Pubkey,      // Dueño del producto
    pub title: String,      // Nombre del producto
    pub price: u64,         // Precio en lamports
    pub is_available: bool, // Disponibilidad
    pub bump: u8,           // Bump de la PDA
}

impl StoreItem {
    // Cálculo del espacio necesario para la cuenta
    // 8  -> Discriminador
    // 32 -> Pubkey
    // 4 + 32 -> String (máx 32 chars)
    // 8  -> u64
    // 1  -> bool
    // 1  -> u8
    pub const LEN: usize = 8 + 32 + 36 + 8 + 1 + 1;
}

/* -------------------------------------------------------------------------- */
/*                               CONTEXTOS (PDAs)                             */
/* -------------------------------------------------------------------------- */

#[derive(Accounts)]
#[instruction(title: String)]
pub struct AddItem<'info> {
    #[account(
        init,
        payer = owner,
        space = StoreItem::LEN,
        seeds = [
            b"item_v1",
            owner.key().as_ref(),
            title.as_bytes()
        ],
        bump
    )]
    pub store_item: Account<'info, StoreItem>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateItem<'info> {
    #[account(
        mut,
        has_one = owner @ LumenError::Unauthorized,
        seeds = [
            b"item_v1",
            owner.key().as_ref(),
            store_item.title.as_bytes()
        ],
        bump = store_item.bump
    )]
    pub store_item: Account<'info, StoreItem>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteItem<'info> {
    #[account(
        mut,
        close = owner,
        has_one = owner @ LumenError::Unauthorized,
        seeds = [
            b"item_v1",
            owner.key().as_ref(),
            store_item.title.as_bytes()
        ],
        bump = store_item.bump
    )]
    pub store_item: Account<'info, StoreItem>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

/* -------------------------------------------------------------------------- */
/*                                   ERRORES                                  */
/* -------------------------------------------------------------------------- */

#[error_code]
pub enum LumenError {
    #[msg("No tienes permisos para modificar este producto.")]
    Unauthorized,

    #[msg("El título debe tener entre 1 y 32 caracteres.")]
    InvalidTitleLength,

    #[msg("El precio debe ser mayor a 0.")]
    InvalidPrice,
}
