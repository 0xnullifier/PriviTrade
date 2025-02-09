package circuit

import (
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/std/algebra/emulated/sw_emulated"
	"github.com/consensys/gnark/std/math/emulated"
	"github.com/consensys/gnark/std/signature/ecdsa"
)

const (
	circuitSize = 5
)

type ProofOfAssets[T, S emulated.FieldParams] struct {
	Balances   [circuitSize]frontend.Variable     `gnark:",public"`
	Addresses  [circuitSize]ecdsa.PublicKey[T, S] `gnark:",public"`
	MinBalance frontend.Variable                  `gnark:",public"`
	MaxBalance frontend.Variable                  `gnark:",public"`

	// Signature
	Sig      ecdsa.Signature[S]             `gnark:",secret"`
	Msg      emulated.Element[S]            `gnark:",secret"`
	Selector [circuitSize]frontend.Variable `gnark:",public"`
}

func (circuit *ProofOfAssets[T, S]) Define(api frontend.API) error {
	field, err := emulated.NewField[T](api)
	if err != nil {
		return err
	}
	var myKey ecdsa.PublicKey[T, S]
	for i := 0; i < circuitSize; i++ {
		selector := field.FromBits(circuit.Selector[i])
		checkedBalances := api.Select(circuit.Selector[i], circuit.Balances[i], circuit.MinBalance)
		myKey.X = *field.Add(&myKey.X, field.Mul(selector, &circuit.Addresses[i].X))
		myKey.Y = *field.Add(&myKey.Y, field.Mul(selector, &circuit.Addresses[i].Y))
		api.AssertIsLessOrEqual(checkedBalances, circuit.MaxBalance)
		api.AssertIsLessOrEqual(circuit.MinBalance, checkedBalances)
	}

	// Verfiy the key
	myKey.Verify(api, sw_emulated.GetCurveParams[T](), &circuit.Msg, &circuit.Sig)

	return nil
}
