import { css } from "@emotion/react";
import styled from "@emotion/styled";
import { Button } from "baseui/button";
import { FC } from "react";
import { ServiceStatus } from "../../Types/ServiceStatus";
import { StopFill } from "@styled-icons/bootstrap/StopFill";
import { Reload } from "@styled-icons/ionicons-outline/Reload";
import { Play } from "@styled-icons/fa-solid/Play";
import _ from "lodash";
import { Spinner } from "baseui/spinner";

const Container = styled.div``;

const StatusTable = styled.div`
  margin-top: 20px;
`;

const StatusRow = styled.div`
  display: flex;
  align-items: flex-start;
  flex-direction: row;
  margin-bottom: 10px;
  flex: 1;
`;

const StatusName = styled.div`
  width: 90px;
  display: flex;
  justify-content: flex-end;
  margin-right: 20px;
  color: #630be2;
`;

const State = styled.div`
  margin-right: 15px;
  color: #630be2;
`;

const StateRow = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
`;

const StatusValue = styled.div<{ terminal?: boolean; nowrap?: boolean }>`
  flex: 1;
  ${({ nowrap }) =>
    nowrap &&
    css`
      max-width: 100%;
      overflow: auto;
      white-space: nowrap;
    `}
  ${(props) =>
    props.terminal &&
    css`
      background-color: #000;
      color: #fff;
      font-family: Ubuntu, monospace;
      padding-left: 10px;
      padding-right: 10px;
    `}
`;

const Actions = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-end;
`;

const CurrentStatus = styled.span<{ color: string }>`
  ${({ color }) =>
    css`
      color: ${color};
    `}
  margin-right: 5px;
`;

const parseStatus = (status: ServiceStatus) => {
  if (status.status.toString().startsWith("Running")) {
    return (
      <StatusValue>
        <CurrentStatus color="#00e667">Running</CurrentStatus>{" "}
        {status.status.toString().replace("Running", "")}
      </StatusValue>
    );
  }
  return <StatusValue>{status.status}</StatusValue>;
};

export interface StatusProps {
  statuses: ServiceStatus[];
  onRestart: () => void;
  onStop: () => void;
  onStart: () => void;
}

const Status: FC<StatusProps> = (props) => {
  const { statuses, onRestart, onStop, onStart } = props;
  const status = statuses.find((status) => status.name === "Active")?.status;
  return (
    <Container>
      <Actions>
        {status?.toString().startsWith("Running") && (
          <>
            <Button
              onClick={onStop}
              startEnhancer={() => <StopFill size={16} color="#630be2" />}
              overrides={{
                BaseButton: {
                  style: {
                    height: "30px",
                    width: "80px",
                    fontSize: "12px",
                    padding: "0px",
                    backgroundColor: "#fff",
                    color: "#630be2",
                    fontFamily: "RockfordSansMedium",
                    marginRight: "10px",
                    borderRadius: "2px",
                    border: "2px solid #630be2",
                    ":hover": {
                      backgroundColor: "#fff",
                      opacity: 0.6,
                    },
                  },
                },
                StartEnhancer: {
                  style: {
                    marginRight: "8px",
                  },
                },
              }}
            >
              Stop
            </Button>
            <Button
              onClick={onRestart}
              startEnhancer={() => <Reload size={14} color="#fff" />}
              overrides={{
                BaseButton: {
                  style: {
                    height: "30px",
                    width: "80px",
                    fontSize: "12px",
                    padding: "0px",
                    fontFamily: "RockfordSansMedium",
                    backgroundColor: "#630be2",
                    color: "#fff",
                    borderRadius: "2px",
                    ":hover": {
                      backgroundColor: "#630be2",
                      opacity: 0.8,
                    },
                  },
                },
                StartEnhancer: {
                  style: {
                    marginRight: "8px",
                  },
                },
              }}
            >
              Restart
            </Button>
          </>
        )}
        {!status?.toString().startsWith("Running") &&
          status?.toString() !== "Starting" &&
          status?.toString() !== "Stopping" && (
            <>
              <Button
                onClick={onStart}
                startEnhancer={() => <Play size={14} color="#fff" />}
                overrides={{
                  BaseButton: {
                    style: {
                      height: "30px",
                      width: "80px",
                      fontSize: "12px",
                      padding: "0px",
                      fontFamily: "RockfordSansMedium",
                      backgroundColor: "#630be2",
                      color: "#fff",
                      borderRadius: "2px",
                      ":hover": {
                        backgroundColor: "#630be2",
                      },
                    },
                  },
                  StartEnhancer: {
                    style: {
                      marginRight: "8px",
                    },
                  },
                }}
              >
                Start
              </Button>
            </>
          )}
        {status?.toString() === "Starting" && (
          <StateRow>
            <State>Starting</State>
            <Spinner $size={"18px"} $borderWidth="3px" $color="#630be2" />
          </StateRow>
        )}
        {status?.toString() === "Stopping" && (
          <StateRow>
            <State>Stopping</State>
            <Spinner $size={"18px"} $borderWidth="3px" $color="#630be2" />
          </StateRow>
        )}
      </Actions>
      <StatusTable>
        {statuses.map((status) => (
          <StatusRow key={_.uniqueId()}>
            <StatusName>{status.name} :</StatusName>
            {status.name !== "Active" && (
              <StatusValue
                terminal={status.name.toLowerCase() === "command"}
                nowrap
              >
                {status.status?.toString()}
              </StatusValue>
            )}
            {status.name === "Active" && parseStatus(status)}
          </StatusRow>
        ))}
      </StatusTable>
    </Container>
  );
};

Status.defaultProps = {
  statuses: [],
};

export default Status;
