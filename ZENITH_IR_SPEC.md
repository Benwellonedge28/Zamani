# Zamani IR Specification

## Overview

The Zamani Intermediate Representation (IR) is a typed, SSA-form low-level language used as the bridge between the frontend AST and target code generation.

## IR Instruction Set

- `Add(dst, lhs, rhs)` — integer/float addition
- `Sub(dst, lhs, rhs)` — integer/float subtraction
- `Mul(dst, lhs, rhs)` — multiplication
- `Div(dst, lhs, rhs)` — division (panics on zero)
- `Not(dst, src)` — boolean/bitwise NOT
- `Eq(dst, lhs, rhs)` — equality comparison
- `Lt(dst, lhs, rhs)` — less-than comparison
- `And(dst, lhs, rhs)` — logical AND
- `Or(dst, lhs, rhs)` — logical OR
- `Call(dst, fn_name, args)` — function call
- `Return(value)` — function return
- `Phi(dst, branches)` — SSA phi node
- `Nop` — no operation

## IR Values

- `ConstInt(i64)` — integer constant
- `ConstFloat(f64)` — float constant
- `ConstBool(bool)` — boolean constant
- `ConstStr(String)` — string constant
- `Reg(IrRegister)` — virtual register reference

## Module Structure

Each `IrModule` contains a list of `IrFunction`s. Each function holds a name, parameter list, return type, and a flat vector of `IrInstruction`s.
