import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TiendaRopa } from "../target/types/tienda_ropa"; // generado por Anchor

async function ejecutarTienda() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TiendaRopa as Program<TiendaRopa>;
  const owner = provider.wallet;

  // PDA de la tienda
  const [tiendaPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("tienda"), owner.publicKey.toBuffer()],
    program.programId
  );

  console.log("Iniciando operaciones de tienda...");

  // 1. Crear tienda
  try {
    await program.methods
      .crearTienda("Mi Tienda de Ropa")
      .accounts({
        tienda: tiendaPda,
        owner: owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Tienda creada con éxito.");
  } catch (e) {
    console.log("La tienda ya existe, continuando...");
  }

  // 2. Agregar producto
  await program.methods
    .agregarProducto("Camisa Azul", 50)
    .accounts({
      tienda: tiendaPda,
      owner: owner.publicKey,
    })
    .rpc();
  console.log("Producto agregado!");

  // 3. Ver productos
  const cuenta = await program.account.tienda.fetch(tiendaPda);
  console.log("Productos en stock:", cuenta.productos);

  // 4. Alternar disponibilidad
  await program.methods
    .alternarDisponibilidad("Camisa Azul")
    .accounts({
      tienda: tiendaPda,
      owner: owner.publicKey,
    })
    .rpc();
  console.log("Disponibilidad actualizada.");
}

// Ejecutar script
ejecutarTienda().catch((err) => {
  console.error("Error en la ejecución:", err);
});
