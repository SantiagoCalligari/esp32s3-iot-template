// Asegúrate de que el archivo contiene EXACTAMENTE esto:

pub mod ap;
pub mod net;

// ✓ Re-exportar las tareas para que sean visibles
pub use ap::connection;
pub use net::net_task;

