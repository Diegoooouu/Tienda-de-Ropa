use anchor_lang::prelude::*;

declare_id!("CPydZL5gyh3Tk5Zj28tnJrF1XbVefPgzpQu5VKjhEDRF");

#[program]
pub mod tienda_ropa {
    use super::*;

    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let productos: Vec<Ropa> = Vec::new();

        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id,
            nombre,
            productos,
        });

        Ok(())
    }

    pub fn agregar_producto(context: Context<NuevoProducto>, nombre: String, precio: u16) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let ropa = Ropa {
            nombre,
            precio,
            disponible: true,
        };

        context.accounts.tienda.productos.push(ropa);

        Ok(())
    }

    pub fn eliminar_producto(context: Context<NuevoProducto>, nombre: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut context.accounts.tienda.productos;

        for i in 0..productos.len() {
            if productos[i].nombre == nombre {
                productos.remove(i);
                msg!("Producto {} eliminado!", nombre);
                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }

    pub fn ver_productos(context: Context<NuevoProducto>) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Lista de productos: {:#?}", context.accounts.tienda.productos);
        Ok(())
    }

    pub fn alternar_disponibilidad(context: Context<NuevoProducto>, nombre: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut context.accounts.tienda.productos;

        for i in 0..productos.len() {

            let estado = productos[i].disponible;

            if productos[i].nombre == nombre {

                let nuevo_estado = !estado;
                productos[i].disponible = nuevo_estado;

                msg!("El producto {} ahora tiene disponibilidad: {}", nombre, nuevo_estado);

                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }

    pub fn total_productos(context: Context<NuevoProducto>) -> Result<()> {

        let total = context.accounts.tienda.productos.len();

        msg!("La tienda tiene {} productos registrados", total);

        Ok(())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,

    #[msg("El producto no existe")]
    ProductoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Tienda {

    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    productos: Vec<Ropa>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Ropa {

    #[max_len(60)]
    nombre: String,

    precio: u16,

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoProducto<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}
