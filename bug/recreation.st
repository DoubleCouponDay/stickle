{external}
TYPE Type1
    : STRUCT
        field1: BOOL;
    END_STRUCT
END_TYPE

{external}
VAR_GLOBAL
    test1: Type1;
END_VAR

PROGRAM BugRecreation
    VAR_EXTERNAL
        test1: Type1; //Could not resolve reference to BugRecreation.test1 at: :.
    END_VAR
    test1.field1 := TRUE; 
END_PROGRAM
