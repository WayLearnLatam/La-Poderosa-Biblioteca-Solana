use anchor_lang::prelude::*;

declare_id!("GkW8S8pj6ZJAfAETJmcuLn5cABBUrt6X8udYVRHyeJar");

#[program] // El codigo empieza desde aqui
pub mod biblioteca {
    use super::*; // Importa todas los structs y enums definidos fuera del modulo

    /////////////////////////// INSTRUCCIONES ///////////////////////////
    /////////////////////////// Crear Biblioteca ///////////////////////////
    pub fn crear_biblioteca(context: Context<NuevaBiblioteca>, n_biblioteca: String) -> Result<()> {

        let owner_id = context.accounts.owner.key(); // caller wallet 

        let libros = Vec::<Pubkey>::new(); // crear un vector vacio 

        context.accounts.biblioteca.set_inner(Biblioteca { 
            owner: owner_id,
            n_biblioteca: n_biblioteca.clone(),
            libros,
        }); // crear el struct de la biblioteca, lo serializa y lo guarda en el espacio de la cuenta (su uso se recomienda cuando se crea una cuenta)

        msg!("Biblioteca {}, creada exitosamente!. Owner id: {}", n_biblioteca.clone(), owner_id); // Log de verificacion

        Ok(())
    }

    /////////////////////////// Nuevo Libro ///////////////////////////
    pub fn agregar_libro(context: Context<NuevoLibro>, nombre: String, paginas: u16) -> Result<()> {
        
        require!(
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        ); // Medida de seguridad 

        let libro = Libro {
            biblioteca: context.accounts.biblioteca.n_biblioteca.clone(),
            nombre: nombre.clone(),
            paginas,
            disponible: true,
        }; // Creacion del struct libro 

        context.accounts.libro.set_inner(libro); // Serializa y guarda el struct en el espacio de la cuenta

        context
            .accounts
            .biblioteca
            .libros
            .push(context.accounts.libro.key()); // Agrega el PDA del libro al vector de libros de biblioteca

        msg!("Libro {}, creado exitosamente, en la biblioteca {}!. Owner id: {}", nombre.clone(),  context.accounts.biblioteca.n_biblioteca, context.accounts.owner.key()); // Log de verificacion
    
        Ok(())
    }

    /////////////////////////// Eliminar Libro ///////////////////////////
    pub fn eliminar_libro(context: Context<EliminarLibro>, nombre: String) -> Result<()> {
        require!(
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        ); // Medida de seguridad 

        let biblioteca = &mut context.accounts.biblioteca;
        let libros = &biblioteca.libros;

        // Verificar que el libro pertenece a esta biblioteca
        require!(
            context.accounts.libro.biblioteca == biblioteca.n_biblioteca,
            Errores::LibroNoPertenece
        );

        require!(biblioteca.libros.contains(&context.accounts.libro.key()), Errores::LibroNoExiste);

        let mut pos = 0;

        for i in 0..libros.len() {
            if libros[i] == context.accounts.libro.key() {
                pos = i;
                break
            }
        }

        // Alternativa mas directa:
        // let pos = biblioteca
        //     .libros
        //     .iter()
        //     .position(|&x| x == context.accounts.libro.key())
        //     .ok_or(Errores::LibroNoExiste)?;

        biblioteca.libros.remove(pos);

        // La cuenta del libro se cierra automáticamente por Anchor debido a 'close = owner'
        msg!("Libro '{}' eliminado exitosamente de la biblioteca {}!. Owner id: {}", nombre, biblioteca.n_biblioteca, context.accounts.owner.key());
            
        Ok(())
    }

    /////////////////////////// Alternar Estado ///////////////////////////
    pub fn alternar_estado(context: Context<ModificarLibro>, nombre: String) -> Result<()> {
        require!(
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let libro = &mut context.accounts.libro;
        let estado = libro.disponible;
        let nuevo_estado = !estado;
        libro.disponible = nuevo_estado;
        
        msg!(
            "El libro: {} ahora tiene un valor de disponibilidad: {}",
            nombre,
            nuevo_estado
        );

        Ok(())
    }
}
/////////////////////////// Codigos de Error ///////////////////////////
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la biblioteca que deseas modificar")]
    NoEresElOwner,
    #[msg("Error, el libro con el que deseas interactuar no existe")]
    LibroNoExiste,
    #[msg("Error, el libro no pertenece a esta biblioteca")]
    LibroNoPertenece,
}


/////////////////////////// CUENTAS ///////////////////////////
/////////////////////////// Biblioteca ///////////////////////////

#[account]
#[derive(InitSpace)]
pub struct Biblioteca {
    pub owner: Pubkey,

    #[max_len(60)]
    pub n_biblioteca: String,

    #[max_len(10)]
    pub libros: Vec<Pubkey>,
}

/////////////////////////// Libro ///////////////////////////

#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct Libro {
    #[max_len(60)]
    pub biblioteca: String,

    #[max_len(60)]
    pub nombre: String,

    pub paginas: u16,

    pub disponible: bool,
}


/////////////////////////// CONTEXTOS ///////////////////////////
/////////////////////////// Nueva Biblioteca ///////////////////////////
/// Instruccion: crear_biblioteca


#[derive(Accounts)]
#[instruction(n_biblioteca:String)]
pub struct NuevaBiblioteca<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner, 
        space = 8 + Biblioteca::INIT_SPACE, 
        seeds = [b"biblioteca", n_biblioteca.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub biblioteca: Account<'info, Biblioteca>,

    pub system_program: Program<'info, System>,
}

/////////////////////////// NuevoLibro ///////////////////////////
/// Instruccion: agregar_libro


#[derive(Accounts)]
#[instruction(nombre:String)]
pub struct NuevoLibro<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner, 
        space = 8 + Libro::INIT_SPACE,
        seeds = [b"libro", nombre.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub libro: Account<'info, Libro>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,

    pub system_program: Program<'info, System>,
}


/////////////////////////// Modificar Libro ///////////////////////////
/// Instruccion: alternar_estado (tambien puede servir para funciones relacionadas con cambiar nombre, numero de paginas o alguna otra variable contenida en el struct Lbro)


#[derive(Accounts)]
pub struct ModificarLibro<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub libro: Account<'info, Libro>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,
}

/////////////////////////// Eliminar Libro ///////////////////////////
///  Instruccion: eliminar_libro -> cierra la cuenta 

#[derive(Accounts)]
pub struct EliminarLibro<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        close = owner,
        constraint = libro.biblioteca == biblioteca.n_biblioteca @ Errores::LibroNoPertenece
    )]
    pub libro: Account<'info, Libro>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,
}