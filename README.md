# Pass-Merger

A crude tool to convert from Bitwarden json format to lastpass.

## Steps to Deduplicate entries from Bitwarden
1. Export all passwords from Chrome, Brave, Firefox etc.
2. Import all to Bitwarden.
3. Export everything from Bitwarden in JSON format and rename it to `bitwarden.json`.
4. Run tool to get `merged.csv`.
5. Purge Bitwarden vault.
6. Import `merged.csv` using LastPass format.
   
### In six easy step you have imported and de-duplicated every entry in Bitwarden.