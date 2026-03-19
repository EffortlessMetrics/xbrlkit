# PR 3: Remaining DEI Required Facts Implementation

## Research Summary

Based on SEC EDGAR Filer Manual (Volume II), sections 6.5.20-6.5.54, the following DEI facts are required for 10-K filings:

### Document Information (6.5.20)
Already implemented:
- ✅ dei:DocumentType
- ✅ dei:DocumentPeriodEndDate

Additional required:
- dei:AmendmentFlag (boolean - true if amendment)
- dei:AmendmentDescription (required only if AmendmentFlag is true)

### Entity Information (6.5.21)
Already implemented:
- ✅ dei:EntityRegistrantName

Additional required:
- dei:EntityCentralIndexKey (CIK number)
- dei:CurrentFiscalYearEndDate (MM-DD format)

### Cover Page Elements (6.5.49)
For 10-K specifically:
- dei:DocumentAnnualReport (boolean true for 10-K)
- dei:EntityWellKnownSeasonedIssuer (WKSI status - Yes/No)

### Entity Address (6.5.48 table)
Required for 10-K:
- dei:EntityAddressAddressLine1
- dei:EntityAddressCityOrTown
- dei:EntityAddressStateOrProvince (or EntityAddressCountry for non-US)
- dei:EntityAddressPostalZipCode

Optional but common:
- dei:EntityAddressAddressLine2

### Auditor Information (6.5.54)
Required for 10-K, 20-F, 40-F for periods ending after Dec 15, 2021:
- dei:AuditorName
- dei:AuditorFirmId (PCAOB Firm ID)
- dei:AuditorLocation (city, state/province, country)

## Implementation Plan

1. Update `profiles/sec/efm-77/opco/required_facts.yaml` with complete list
2. Create new test fixtures with all required facts
3. Update BDD scenarios to test complete validation
4. Verify alpha-check passes

## Required Facts for 10-K (Complete List)

```yaml
required_facts:
  # Document Information (6.5.20)
  - dei:DocumentType
  - dei:DocumentPeriodEndDate
  - dei:AmendmentFlag
  
  # Entity Information (6.5.21)
  - dei:EntityRegistrantName
  - dei:EntityCentralIndexKey
  - dei:CurrentFiscalYearEndDate
  
  # Cover Page - Annual Report (6.5.49)
  - dei:DocumentAnnualReport
  
  # Entity Address (6.5.48)
  - dei:EntityAddressAddressLine1
  - dei:EntityAddressCityOrTown
  - dei:EntityAddressStateOrProvince
  - dei:EntityAddressPostalZipCode
  
  # Auditor Information (6.5.54)
  - dei:AuditorName
  - dei:AuditorFirmId
  - dei:AuditorLocation
```

## Notes

- AmendmentDescription is conditionally required (only if AmendmentFlag=true) - may need special handling
- EntityAddressCountry is an alternative to StateOrProvince for non-US companies
- Auditor facts are only required for DEI 2021Q4+ taxonomy versions
- Some facts like EntityWellKnownSeasonedIssuer are technically optional but commonly present
