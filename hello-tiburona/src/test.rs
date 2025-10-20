#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, Symbol};
use soroban_sdk::testutils::Address;

/// Test 1: Verificación de inicialización del contrato
/// 
/// Este test verifica que:
/// - El contrato se puede inicializar correctamente con una dirección admin
/// - El contador de saludos se inicializa en 0
/// - La función initialize funciona sin errores
#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    // Crear una dirección de prueba para el admin
    let admin = <soroban_sdk::Address as Address>::generate(&env);
    
    // Inicializar el contrato - debe funcionar sin errores
    client.initialize(&admin);
    
    // Verificar que el contador se inicializa en 0
    assert_eq!(client.get_contador(), 0);
}

/// Test 2: Verificación de funcionalidad principal del contrato
/// 
/// Este test verifica que:
/// - La función hello funciona correctamente con un admin autorizado
/// - Retorna el símbolo "Hola" como se espera
/// - Incrementa el contador de saludos
/// - Guarda el último saludo del usuario en almacenamiento persistente
#[test]
fn test_hello_exitoso() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let admin = <soroban_sdk::Address as Address>::generate(&env);
    
    // Inicializar el contrato primero
    client.initialize(&admin);
    
    // Llamar a la función hello con el admin (usuario autorizado)
    let result = client.hello(&admin, &String::from_str(&env, "Dev"));
    
    // Verificar que devuelve el símbolo "Hola" como se espera
    assert_eq!(result, Symbol::new(&env, "Hola"));
    
    // Verificar que el contador se incrementó de 0 a 1
    assert_eq!(client.get_contador(), 1);
    
    // Verificar que se guardó el último saludo del usuario
    assert_eq!(client.get_ultimo_saludo(&admin), String::from_str(&env, "Dev"));
}

/// Test 3: Validación de nombre vacío
/// 
/// Este test verifica que:
/// - El contrato rechaza nombres vacíos (string de longitud 0)
/// - Retorna el error correcto (Error #1 = NombreVacio)
/// - La función hello falla apropiadamente con entrada inválida
#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_nombre_vacio() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);
    
    let admin = <soroban_sdk::Address as Address>::generate(&env);
    
    // Inicializar el contrato
    client.initialize(&admin);
    
    // Intentar usar un string vacío - debe fallar con Error #1 (NombreVacio)
    let vacio = String::from_str(&env, "");
    client.hello(&admin, &vacio);  // Esta llamada debe causar panic
}

/// Test 4: Prevención de reinicialización
/// 
/// Este test verifica que:
/// - El contrato no puede ser inicializado dos veces
/// - La segunda llamada a initialize falla con el error correcto
/// - El estado del contrato se mantiene consistente
#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_no_reinicializar() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);
    
    let admin = <soroban_sdk::Address as Address>::generate(&env);
    
    // Primera inicialización debe funcionar
    client.initialize(&admin);
    
    // Segunda inicialización debe fallar con Error #4 (NoInicializado)
    client.initialize(&admin);  // Esta llamada debe causar panic
}

/// Test 5: Funcionalidad de reset para administradores
/// 
/// Este test verifica que:
/// - El admin puede resetear el contador de saludos
/// - El contador se resetea correctamente a 0
/// - Solo los administradores tienen este privilegio
#[test]
fn test_reset_solo_admin() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);
    
    let admin = <soroban_sdk::Address as Address>::generate(&env);
    
    // Inicializar el contrato
    client.initialize(&admin);
    
    // Hacer un saludo para incrementar el contador
    client.hello(&admin, &String::from_str(&env, "Test"));
    assert_eq!(client.get_contador(), 1);
    
    // El admin debe poder resetear el contador
    client.reset_contador(&admin);
    
    // Verificar que el contador se reseteó a 0
    assert_eq!(client.get_contador(), 0);
}

/// Test 6: Control de acceso para función de reset
/// 
/// Este test verifica que:
/// - Solo el admin puede resetear el contador
/// - Usuarios no autorizados reciben el error correcto
/// - El sistema de permisos funciona correctamente
#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_reset_no_autorizado() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);
    
    let admin = <soroban_sdk::Address as Address>::generate(&env);
    let otro_usuario = <soroban_sdk::Address as Address>::generate(&env);
    
    // Inicializar el contrato con el admin
    client.initialize(&admin);
    
    // Un usuario no admin intenta resetear el contador
    // Debe fallar con Error #3 (NoAutorizado)
    client.reset_contador(&otro_usuario);  // Esta llamada debe causar panic
}
