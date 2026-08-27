# Backlog

## Tracing

Parser / Lexer tracing for debugging.

```
-> parse_expr
  -> parse_binary_expr
    -> parse_primary
    <- parse_primary: LitInt(42)
  <- parse_binary_expr
<- parse_expr
```

