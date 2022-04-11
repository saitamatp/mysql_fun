# Ledger Control

* This Code will be called by C# WPF to Load and process data in MYSQL database.
* PRM_Reader will read the file and connection details from a flat file and store it in Hashmap.
* Main function will create the connection string and call different functions based on the arguments passed.
    * If argument is 1 - Data will be read from CSV file and loaded to temporary table(Expense Load).
    * If argument is 2 - Data will be read from temporary Tables and loaded to base tables.
    * If argument is 3 - Successfully imported/updated records will be cleared.
    * If argument is 4 - Data will be read from CSV file and loaded to base table(Credit details).
    * If argument is 5 - Period end Stored procedure will be called. Link-https://github.com/saitamatp/Misc/blob/main/sp_period_end_cal.sql
    * If argument is 100 - This is to check if C# is triggering this code (C# connection Check).
    * Any other argument for now is not recognised.