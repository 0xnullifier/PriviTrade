package circuit

import (
	"crypto/rand"
	"math/big"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark-crypto/ecc/secp256k1/ecdsa"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/consensys/gnark/std/math/emulated"
	gnarkEcdsa "github.com/consensys/gnark/std/signature/ecdsa"
	"github.com/consensys/gnark/test"
)

type Secp256k1Fp = emulated.Secp256k1Fp
type Secp256k1Fr = emulated.Secp256k1Fr

func TestProofOfAssetsWithSelector(t *testing.T) {
	assert := test.NewAssert(t)

	// Create private keys and addresses for all positions
	privateKeys := make([]*ecdsa.PrivateKey, circuitSize)
	var addresses [circuitSize]gnarkEcdsa.PublicKey[Secp256k1Fp, Secp256k1Fr]

	for i := 0; i < circuitSize; i++ {
		// Generate parameters
		privKey, err := ecdsa.GenerateKey(rand.Reader)
		if err != nil {
			t.Fatal(err)
		}
		privateKeys[i] = privKey
		addresses[i] = gnarkEcdsa.PublicKey[Secp256k1Fp, Secp256k1Fr]{
			X: emulated.ValueOf[Secp256k1Fp](privKey.PublicKey.A.X),
			Y: emulated.ValueOf[Secp256k1Fp](privKey.PublicKey.A.Y),
		}
	}

	// Create balances array
	var balances [circuitSize]frontend.Variable
	balances = [circuitSize]frontend.Variable{
		2000, 1500, 2500, 1800, 2200,
	}

	// Test 1: Valid case - prove ownership of single address
	targetIndex := 2 // We'll prove ownership of address 2
	msg := []byte("testing ECDSA (pre-hashed)")

	// Sign message
	sigBin, err := privateKeys[targetIndex].Sign(msg, nil)
	if err != nil {
		t.Fatal(err)
	}

	// Verify signature
	flag, err := privateKeys[targetIndex].PublicKey.Verify(sigBin, msg, nil)
	if err != nil || !flag {
		t.Fatal("can't verify signature")
	}

	// Unmarshal signature
	var sig ecdsa.Signature
	sig.SetBytes(sigBin)
	r, s := new(big.Int), new(big.Int)
	r.SetBytes(sig.R[:32])
	s.SetBytes(sig.S[:32])
	hash := ecdsa.HashToInt(msg)

	// Create selector array (all zeros except 1 for target address)
	var selector [circuitSize]frontend.Variable
	for i := 0; i < circuitSize; i++ {
		if i == targetIndex {
			selector[i] = 1
		} else {
			selector[i] = 0
		}
	}

	validAssignment := ProofOfAssets[Secp256k1Fp, Secp256k1Fr]{
		Balances:   balances,
		Addresses:  addresses,
		MinBalance: 1000,
		MaxBalance: 3000,
		Sig: gnarkEcdsa.Signature[Secp256k1Fr]{
			R: emulated.ValueOf[Secp256k1Fr](r),
			S: emulated.ValueOf[Secp256k1Fr](s),
		},
		Msg:      emulated.ValueOf[Secp256k1Fr](hash),
		Selector: selector,
	}

	circuit := ProofOfAssets[Secp256k1Fp, Secp256k1Fr]{}
	err = test.IsSolved(&circuit, &validAssignment, ecc.BN254.ScalarField())
	assert.NoError(err)

	// Test 2: Invalid case - wrong selector
	invalidSelector := selector
	invalidSelector[targetIndex] = 0
	invalidSelector[3] = 1 // Try to claim ownership of address 3

	invalidAssignment := ProofOfAssets[Secp256k1Fp, Secp256k1Fr]{
		Balances:   balances,
		Addresses:  addresses,
		MinBalance: 1000,
		MaxBalance: 3000,
		Sig: gnarkEcdsa.Signature[Secp256k1Fr]{
			R: emulated.ValueOf[Secp256k1Fr](r),
			S: emulated.ValueOf[Secp256k1Fr](s),
		},
		Msg:      emulated.ValueOf[Secp256k1Fr](hash),
		Selector: invalidSelector,
	}

	err = test.IsSolved(&circuit, &invalidAssignment, ecc.BN254.ScalarField())
	if err == nil {
		t.Fatal("invalid case (wrong selector) should have failed")
	}

	// Test 3: Invalid case - multiple selections
	multipleSelector := selector
	multipleSelector[1] = 1 // Select additional address

	multipleSelectionAssignment := ProofOfAssets[Secp256k1Fp, Secp256k1Fr]{
		Balances:   balances,
		Addresses:  addresses,
		MinBalance: 1000,
		MaxBalance: 3000,
		Sig: gnarkEcdsa.Signature[Secp256k1Fr]{
			R: emulated.ValueOf[Secp256k1Fr](r),
			S: emulated.ValueOf[Secp256k1Fr](s),
		},
		Msg:      emulated.ValueOf[Secp256k1Fr](hash),
		Selector: multipleSelector,
	}

	err = test.IsSolved(&circuit, &multipleSelectionAssignment, ecc.BN254.ScalarField())
	if err == nil {
		t.Fatal("invalid case (multiple selections) should have failed")
	}
}

func TestProofOfAssetsCompile(t *testing.T) {
	var circuit ProofOfAssets[Secp256k1Fp, Secp256k1Fr]

	_, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		t.Fatal("circuit compilation failed:", err)
	}
}
