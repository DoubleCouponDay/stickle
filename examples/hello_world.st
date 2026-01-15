{external}
FUNCTION puts : DINT
VAR_INPUT {ref}
    text : STRING;
END_VAR
END_FUNCTION

FUNCTION main : DINT
    WHILE TRUE DO
        puts('hello, world!$N');
    END_WHILE;
END_FUNCTION
