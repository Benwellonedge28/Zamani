NIMBUS Grammar v2.0 — UNIVERSAL TRINITY EDITION — ANTLR4 format.
Inherits all 847 original rules PLUS absorbs ALL Zenith features PLUS adds Sankofa memory features.

NEW RULES ADDED FROM ZENITH:
- quantum_circuit_decl (top-level quantum circuit syntax)
- quantum_lit: '|' QUBIT_STATE '⟩' (Dirac notation literals)
- nano_agent_decl (nano agent top-level declaration)
- nano_lit: '@atom(ELEMENT:ORBITAL)' | '@molecule(FORMULA)'
- mts_lit: 'mts[' INTEGER_LIT ']' 
- language_decl (meta-compilation — define new languages)
- effect_decl / handle_expr (algebraic effects)
- Quantum types: Qubit | QReg[N] | Superposition<T> | Entangled<A,B> | QMeasured<T>
- Nano types: Atom<E> | Molecule<F> | NanoAgent<C> | Archaeve<T>
- Universe types: Type_N | Kind | Sort | Prop
- MTS types: MtsSlice<N>
- Pi/Sigma/Identity types: Π(x:T)T | Σ(x:T)T | Id(T,e,e)
- linear T | affine T (linear/affine types)
- evas_cert: unsafe!(evas:{proof}) block

NEW RULES ADDED FROM SANKOFA:
- memory_decl: 'remember' IDENTIFIER ':' type_expr '=' expr ';'
- recall_expr: 'recall' '(' domain_expr ',' context_expr ')'
- learn_stmt: 'learn' 'from' expr ['with' 'weight' expr] ';'
- wisdom_decl: 'wisdom' IDENTIFIER '{' wisdom_body '}'
- ancestor_call: 'ancestral' IDENTIFIER '(' args ')'
- consensus_expr: 'consensus' '[' expr_list ']' 'vote' expr
- zamani_block: 'zamani' '{' stmts '}'
- sasa_block: 'sasa' '{' stmts '}'
- history_type: 'History' '<' type ',' years_expr '>'
- consensus_type: 'ConsensusTrue' '<' type '>'
- inter_memory: 'InterMemory' '<' lang_id ',' type '>'
- sankofa_observe: '@observe' '(' scope ')'
- living_doc: '@living_doc' '(' update_policy ')'
- temporal_learn: '@temporal_learn' '(' span ')'

TOTAL RULES: ~1,100 (NIMBUS v2.0 Trinity Edition)
KEYWORDS: 140 total (95 original + 25 from Zenith + 20 from Sankofa)
ALL paradigms: 80 (original 71 + 9 new Sankofa paradigms)