TYPE State : (
    AllStopped,
    ClampsExtending,
    ClampsExtended,
    SawExtending,
    SawExtended,
    SawRetracting,
    SawRetracted,
    ClampsRetracting,
    ClampsRetracted
);
END_TYPE

VAR_GLOBAL
    currentState : State := AllStopped;
    THNTD : BOOL := FALSE;
END_VAR

{external}
FUNCTION puts : DINT
VAR_INPUT {ref}
    text : STRING;
END_VAR
END_FUNCTION

FUNCTION_BLOCK StateMachine
    CASE currentState OF
        AllStopped:
            IF THNTD = FALSE THEN
                
            ELSE
            
            END_IF;
            currentState := ClampsExtending;

        ClampsExtending:
            IF THNTD THEN
                currentState := ClampsExtended;
            ELSE
                currentState := AllStopped;
            END_IF;

        ClampsExtended:
            IF THNTD THEN
                currentState := SawExtending;
            ELSE
                currentState := ClampsExtending;
            END_IF;

        SawExtending:
            IF THNTD THEN
                currentState := SawExtended;
            ELSE
                currentState := ClampsExtended;
            END_IF;

        SawExtended:
            IF THNTD THEN
                currentState := SawRetracting;
            ELSE
                currentState := SawExtending;
            END_IF;

        SawRetracting:
            currentState := SawRetracted;

        SawRetracted:
            currentState := ClampsRetracting;

        ClampsRetracting:
            currentState := ClampsRetracted;

        ClampsRetracted:
            currentState := AllStopped;
    END_CASE;
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    sm : StateMachine;
END_VAR
    sm();
END_FUNCTION
