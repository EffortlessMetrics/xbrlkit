# XBRL Dimensions Research Notes

## Overview
Research into the XBRL Dimensions specification to implement context parsing for dimensional validation.

## Key Resources

### Primary Specifications
- **XBRL Dimensions 1.0** - The core dimension specification
- **XBRL Formula 1.0** - Validation rules using dimensions
- **XBRL Table Linkbase** - Presentation of dimensional data

### EDGAR Context
- SEC filers heavily use dimensions for:
  - **Segments** - Business segments, geographic segments
  - **Scenarios** - Forecast vs actual, budget vs actual
  - **Time periods** - Interim periods, YTD vs single period

## Context Structure

### Context Components
```
Context
├── id (required)
├── Entity (required)
│   ├── identifier (scheme + value)
│   └── segment (optional) - contains dimensional info
├── Period (required)
│   ├── instant
│   ├── startDate + endDate
│   └── forever
└── Scenario (optional) - contains dimensional info
```

### Dimension Elements
- **Dimension** (`xbrldt:dimensionItem`) - Abstract, defines the dimension
- **Domain** (`xbrldt:domainItem`) - Abstract, root of the domain hierarchy
- **Member** (`xbrldt:domainItem`) - Concrete, a value in the domain

### Dimensional Taxonomy Arcs
- **hypercube-dimension** - Links hypercube to its dimensions
- **dimension-domain** - Links dimension to its default domain
- **domain-member** - Domain hierarchy (parent-child members)
- **all** - Links concept to hypercube (all dimensions required)
- **notAll** - Links concept to hypercube (some dimensions optional)

## Implementation Targets

### Phase 1: Context Parsing
1. Parse context XML into structured Context object
2. Extract entity identifier
3. Extract period (instant or duration)
4. Extract dimensional info from segment/scenario

### Phase 2: Dimension Resolution
1. Load dimension taxonomy
2. Build dimension-domain-member hierarchy
3. Validate member usage against taxonomy

### Phase 3: Fact Dimensional Validation
1. Match facts to contexts
2. Validate dimension usage per concept
3. Check for required dimensions
4. Validate member values against domain

## Complexity Estimate
- **Phase 1**: 8-12 hours (context parsing)
- **Phase 2**: 16-24 hours (dimension resolution)
- **Phase 3**: 24-36 hours (validation rules)
- **Total**: ~48-72 hours

## Open Questions
1. How to handle typed dimensions vs explicit dimensions?
2. How to handle default dimensions?
3. How to validate dimensional aggregations (roll-up)?
4. What's the priority: segments vs scenarios?

## Next Steps
1. Read XBRL Dimensions specification
2. Create parser for context XML
3. Implement dimension taxonomy loader
4. Build dimensional validation rules
