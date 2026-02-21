use anchor_lang::prelude::*;

declare_id!("5baiYRTMXDsot67BwspG8EX7bFC7GVvyvKBeLowhjhet");

#[program]
pub mod biblioteca {
    use super::*;

    pub fn crear_biblioteca(context: Context<NuevaBiblioteca>, n_biblioteca: String) -> Result<()> {
        
    }

    pub fn agregar_libro(context: Context<NuevoLibro>, nombre: String, paginas: u16) -> Result<()> {
        
    }

    pub fn eliminar_libro(context: Context<EliminarLibro>, nombre: String) -> Result<()> {
        
    }

    pub fn alternar_estado(context: Context<ModificarLibro>, nombre: String) -> Result<()> {
        
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la biblioteca que deseas modificar")]
    NoEresElOwner,
    #[msg("Error, el libro con el que deseas interactuar no existe")]
    LibroNoExiste,
    #[msg("Error, el libro no pertenece a esta biblioteca")]
    LibroNoPertenece,
}

