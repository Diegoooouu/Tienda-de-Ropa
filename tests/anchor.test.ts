describe("Tienda de Ropa", () => {
  it("crear tienda y agregar producto", async () => {
    // Generar keypair para la nueva tienda
    const ownerKp = new web3.Keypair();

    // PDA de la tienda
    const [tiendaPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("tienda"), ownerKp.publicKey.toBuffer()],
      pg.program.programId
    );

    // Inicializar tienda
    const txHash = await pg.program.methods
      .crearTienda("Mi Tienda de Ropa")
      .accounts({
        tienda: tiendaPda,
        owner: ownerKp.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([ownerKp])
      .rpc();

    console.log(`Transacción: ${txHash}`);

    // Confirmar transacción
    await pg.connection.confirmTransaction(txHash);

    // Agregar producto
    await pg.program.methods
      .agregarProducto("Camisa Azul", 50)
      .accounts({
        tienda: tiendaPda,
        owner: ownerKp.publicKey,
      })
      .signers([ownerKp])
      .rpc();

    // Fetch de la cuenta tienda
    const tienda = await pg.program.account.tienda.fetch(tiendaPda);
    console.log("Productos registrados:", tienda.productos);

    // Validar que el producto se agregó
    assert.equal(tienda.productos.length, 1);
    assert.equal(tienda.productos[0].nombre, "Camisa Azul");
  });
});
