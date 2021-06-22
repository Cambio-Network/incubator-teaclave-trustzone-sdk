// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//for ta
pub enum Command {
    Ping,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::Ping,
            _ => Command::Unknown,
        }
    }
}

pub const TA_UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/ta_uuid.txt"));

//for plugin
pub enum PluginCommand {
    Print,
    Unknown,
}

impl From<u32> for PluginCommand {
    #[inline]
    fn from(value: u32) -> PluginCommand {
        match value {
            0 => PluginCommand::Print,
            _ => PluginCommand::Unknown,
        }
    }
}

pub const PLUGIN_SUBCMD_NULL: u32 = 0xFFFFFFFF;
pub const PLUGIN_UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/plugin_uuid.txt"));

