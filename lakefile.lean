import Lake
open Lake DSL

package «anima-core-uplift» where
  version := v!"2.0.0"

@[default_target]
lean_lib «AnimaCore» where
  srcDir := "verification"
  roots := #[`QuantizationBounds]
