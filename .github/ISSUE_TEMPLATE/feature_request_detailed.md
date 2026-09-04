---
name: Feature request (detailed)
about: A worked proposal for a new tolerance kind, property, perturbation or output format.
title: "[feature] <short description>"
labels: ["enhancement"]
assignees: []
---

## Problem

<!--
The regression you cannot catch, or the assertion you cannot express, with the
model as it stands today. Describe the situation, not the solution.
-->

## Why the current model does not cover it

<!--
Which of these did you try, and where did each fall short?
abs / rel tolerances, the seven properties, the three perturbations, a second
test with a different dataset.
-->

## Proposal

<!-- What you would add. Everything here is data, so show the JSON. -->

```jsonc
{
  "tolerances": { },
  "property_checks": [ ],
  "fuzz": { }
}
```

## Semantics

<!--
Be precise about the edge cases, since this becomes a serde variant that every
binding must agree on:
- What does it do when the field is absent? NaN? infinite?
- Is it deterministic across the parallel and sequential paths?
- Does it change the shape of an existing result, or only add to it?
-->

## Compatibility

- [ ] Purely additive (a new enum variant; old tests keep working)
- [ ] Changes the meaning of an existing field
- [ ] Changes the result schema (goldens would need re-blessing)

## Alternatives considered

<!-- Including "do nothing" and why that is not enough. -->

## Willing to implement?

- [ ] Yes, with review guidance
- [ ] No, proposing only
