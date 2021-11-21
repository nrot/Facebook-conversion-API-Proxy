-- Create a new table called 'Token' in schema '
CREATE TABLE IF NOT EXISTS Token 
(
    TokenId INT NOT NULL PRIMARY KEY, -- primary key column
    Column1 [NVARCHAR](50) NOT NULL,
    Column2 [NVARCHAR](50) NOT NULL
    -- specify more columns here
);
GO