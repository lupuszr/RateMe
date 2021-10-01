import React from 'react';
import { Flex, Box } from "rebass";
import {
  TextInputField,
  Text,
  Pane,
  Heading,
  Button,
  Link
} from "evergreen-ui";

export const Header = () => {return (
    <div>
        <Box>
            <Pane
                background="tint1"
                border="default"
                display="flex"
                padding={16}
                marginBottom={30}
            >
                <Pane flex={1} alignItems="center" display="flex">
                <Heading size={600}>Rate Me</Heading>
                </Pane>
                <Pane>
                <Link marginRight={16}>Blog</Link>
                {/* <Link>About</Link> */}
                </Pane>
            </Pane>
        </Box>
    </div>
)}