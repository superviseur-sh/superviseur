import styled from "@emotion/styled";
import { FC, useState } from "react";
import { Node } from "../../Types/Node";
import { Terminal } from "@styled-icons/fa-solid/Terminal";
import { Tabs, Tab } from "baseui/tabs-motion";
import Log from "../Log";
import Variables from "../Variables";
import Settings from "../Settings";
import Status from "../Status";

const DrawerHeader = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  margin-bottom: 2rem;
`;

const ServiceName = styled.div`
  font-size: 24px;
  color: #000;
  margin-left: 20px;
`;

const Container = styled.div`
  height: calc(100% - 150px);
`;
export interface ServiceDetailsProps {
  nodes: Node[];
  selectedNode?: string;
}

const ServiceDetails: FC<ServiceDetailsProps> = (props) => {
  const [activeKey, setActiveKey] = useState(0);
  const { selectedNode } = props;
  return (
    <Container>
      <DrawerHeader>
        <Terminal size={28} color="#000" />
        <ServiceName>
          {props.nodes
            .find((x) => x.id === selectedNode)
            ?.label.split("<b>")[1]
            .replace("</b>", "")}
        </ServiceName>
      </DrawerHeader>
      <Tabs
        activeKey={activeKey}
        onChange={({ activeKey }) => setActiveKey(activeKey as number)}
        overrides={{
          Root: {
            style: ({ $theme }) => ({
              height: "100%",
            }),
          },
        }}
      >
        <Tab title="Status">
          {selectedNode && <Status selectedNode={selectedNode} />}
        </Tab>
        <Tab
          title="Log"
          overrides={{
            TabPanel: {
              style: {
                paddingLeft: "0px",
                paddingRight: "0px",
                paddingBottom: "0px",
                paddingTop: "16px",
                height: "100%",
              },
            },
          }}
        >
          {selectedNode && <Log serviceId={selectedNode} />}
        </Tab>
        <Tab
          title="Variables"
          overrides={{
            TabPanel: {
              style: {
                paddingLeft: "0px",
                paddingRight: "0px",
                paddingBottom: "0px",
                paddingTop: "16px",
                height: "100%",
              },
            },
          }}
        >
          {selectedNode && <Variables serviceId={selectedNode} />}
        </Tab>
        <Tab
          title="Settings"
          overrides={{
            TabPanel: {
              style: {
                paddingLeft: "0px",
                paddingRight: "0px",
                paddingBottom: "0px",
                paddingTop: "16px",
                height: "100%",
              },
            },
          }}
        >
          {selectedNode && <Settings serviceId={selectedNode} />}
        </Tab>
      </Tabs>
    </Container>
  );
};

ServiceDetails.defaultProps = {
  nodes: [],
};

export default ServiceDetails;
