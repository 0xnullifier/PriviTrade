package circuit

import (
	"bytes"
	"encoding/json"
	"log"
	"syscall/js"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	cs "github.com/consensys/gnark/constraint/bn254"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/consensys/gnark/std/math/emulated"
	"github.com/consensys/gnark/test/unsafekzg"
)

func main() {
	c := make(chan struct{}, 0)
	js.Global().Set("generateProof", js.FuncOf(generateProof))
	<-c
}

func generateProof(this js.Value, args []js.Value) interface{} {
	// Parse input from JavaScript
	witnessJSON := []byte(args[0].String())
	var assignment ProofOfAssets[emulated.Secp256k1Fp, emulated.Secp256k1Fr]
	if err := json.NewDecoder(bytes.NewReader(witnessJSON)).Decode(&assignment); err != nil {
		return map[string]interface{}{
			"error": err.Error(),
		}
	}
	fullWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	if err != nil {
		log.Fatal(err)
	}
	publicWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField(), frontend.PublicOnly())
	if err != nil {
		log.Fatal(err)
	}

	// Create circuit
	var circuit ProofOfAssets[emulated.Secp256k1Fp, emulated.Secp256k1Fr]

	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	r1cs := ccs.(*cs.SparseR1CS)
	srs, srsLagrangeInterpolation, err := unsafekzg.NewSRS(r1cs)
	if err != nil {
		panic("KZG setup error")
	}
	// Generate proving key
	if err != nil {
		return map[string]interface{}{
			"error": err.Error(),
		}
	}

	// Generate proof
	pk, vk, _ := plonk.Setup(ccs, srs, srsLagrangeInterpolation)

	if err != nil {
		return map[string]interface{}{
			"error": err.Error(),
		}
	}

	proof, err := plonk.Prove(ccs, pk, fullWitness)
	if err != nil {
		panic("PLONK proof generation error")
	}
	err = plonk.Verify(proof, vk, publicWitness)
	if err != nil {
		panic("PLONK proof not verified")
	}

	// Convert proof to bytes
	var proofBytes bytes.Buffer
	proof.WriteRawTo(&proofBytes)

	return map[string]interface{}{
		"proof": proofBytes.Bytes(),
		"vk":    vk,
	}
}
