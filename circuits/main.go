package main

/*
#include <stdlib.h>
*/
import "C"

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"math/big"

	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/consensys/gnark-crypto/ecc"
)

// Circuit defines x^3 + x + 5 == y
type Circuit struct {
	X frontend.Variable `gnark:"x"`
	Y frontend.Variable `gnark:",public"`
}

// Define declares the circuit constraints
func (c *Circuit) Define(api frontend.API) error {
	x3 := api.Mul(c.X, c.X, c.X)
	api.AssertIsEqual(c.Y, api.Add(x3, c.X, 5))
	return nil
}

//export GenerateProof
func GenerateProof(xValue, yValue int64) *C.char {
	var circuit Circuit

	// Compile the circuit
	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		fmt.Println("Error compiling circuit:", err)
		return C.CString("ERROR")
	}

	// Trusted setup
	pk, vk, err := groth16.Setup(r1cs)
	if err != nil {
		fmt.Println("Error during setup:", err)
		return C.CString("ERROR")
	}
	_ = vk

	// Create witness
	assignment := &Circuit{
		X: big.NewInt(xValue),
		Y: big.NewInt(yValue),
	}

	witness, err := frontend.NewWitness(assignment, ecc.BN254.ScalarField())
	if err != nil {
		fmt.Println("Error creating witness:", err)
		return C.CString("ERROR")
	}

	// Generate the proof
	proof, err := groth16.Prove(r1cs, pk, witness)
	if err != nil {
		fmt.Println("Error generating proof:", err)
		return C.CString("ERROR")
	}

	// Serialize proof to hex
	var buf bytes.Buffer
	proof.WriteTo(&buf)
	proofHex := hex.EncodeToString(buf.Bytes())

	return C.CString(proofHex)
}

func main() {}
