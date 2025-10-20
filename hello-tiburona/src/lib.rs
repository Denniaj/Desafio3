// Imports y setup inicial
#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Address, String, Symbol
};

#[cfg(test)]
mod test;

// Definir errores 
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}

// Definir DataKey
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
}

// Definir el contrato
#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {  //Firma de la función
        if env.storage().instance().has(&DataKey::Admin) {              //Verificación
            return Err(Error::NoInicializado);
        }

        env.storage()                                               //Guardar el admin
            .instance()
            .set(&DataKey::Admin, &admin);

        env.storage()                                              //Inicialización del contador
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);

        env.storage()                                              //Extensión de TTL
            .instance()
            .extend_ttl(100, 100);
        
        Ok(())
    }

    pub fn hello(
        env: Env,
        usuario: Address,
        nombre: String  // ⭐ String en lugar de Symbol
    ) -> Result<Symbol, Error> {

        // Validación 1 - Nombre no vacío
        if nombre.len() == 0 {                                   
            return Err(Error::NombreVacio);
        }

        // Validación 2 - Nombre no muy largo
        if nombre.len() > 32 {
            return Err(Error::NombreMuyLargo);
        }

        // Validación 3 - Usuario no autorizado
        let admin = env.storage().instance().get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;
        if usuario != admin {
            return Err(Error::NoAutorizado);
        }

        // Incrementar contador
        let key_contador = DataKey::ContadorSaludos;
        let contador: u32 = env.storage()
            .instance()
            .get(&key_contador)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&key_contador, &(contador + 1));

        // Guardar el último saludo
        env.storage()
            .persistent()
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);

        // Extender TTL
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);
        
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        // Retornar saludo
        Ok(Symbol::new(&env, "Hola"))
    }

    // Funciones de consulta

    //Get contador 
    pub fn get_contador(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ContadorSaludos)
            .unwrap_or(0)
    }

    //Get ultimo saludo
    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> String {
        env.storage()
            .persistent()
            .get(&DataKey::UltimoSaludo(usuario))
            .unwrap_or(String::from_str(&env, ""))
    }

    // Funciones de administración

    //Reset contador
    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;

            if caller != admin {
                return Err(Error::NoAutorizado);
            }
    
        //Resetear contador
            env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
        
        Ok(())
    }
}

