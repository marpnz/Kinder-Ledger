describe("LUMEN - Pruebas Finales", () => {
  const juegoTitulo = "Elden_Ring";
  const precioInicial = new anchor.BN(1000000); 

  const [itemPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("item_v1"),
      pg.wallet.publicKey.toBuffer(),
      Buffer.from(juegoTitulo),
    ],
    pg.program.programId
  );

  it("Paso 1: Añadir Item", async () => {
    await (pg.program.methods as any)
      .addItem(juegoTitulo, precioInicial)
      .accounts({
        storeItem: itemPda,
        owner: pg.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const cuenta = await (pg.program.account as any).storeItem.fetch(itemPda);
    console.log("✔ Juego creado:", cuenta.title);
  });

  it("Paso 2: Actualizar", async () => {
    await (pg.program.methods as any)
      .updateItem(new anchor.BN(500000), false)
      .accounts({
        storeItem: itemPda,
        owner: pg.wallet.publicKey,
      })
      .rpc();

    const cuenta = await (pg.program.account as any).storeItem.fetch(itemPda);
    console.log("✔ Precio actualizado a:", cuenta.price.toString());
  });

  it("Paso 3: Borrar", async () => {
    await (pg.program.methods as any)
      .deleteItem()
      .accounts({
        storeItem: itemPda,
        owner: pg.wallet.publicKey,
      })
      .rpc();

    try {
      await (pg.program.account as any).storeItem.fetch(itemPda);
    } catch (e) {
      console.log("✔ Cuenta cerrada y renta recuperada.");
    }
  });
});
