// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<N: Network> FromBytes for ArrayType<N> {
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the element type.
        let element_type = ElementType::read_le(&mut reader)?;
        // Read the number of dimensions of the array.
        let num_dimensions = u8::read_le(&mut reader)?;
        // Initialize a buffer for the dimensions of the array.
        // Note: `dimensions` needs to be explicitly specified since Rust does not yet support constant expressions that depends on a generic parameter.
        let mut dimensions = [0; 32];
        for dimension in dimensions.iter_mut().take(std::cmp::min(num_dimensions as usize, N::MAX_DATA_DEPTH)) {
            *dimension = u32::read_le(&mut reader)?;
        }
        // Return the array type.
        ArrayType::new(element_type, dimensions, num_dimensions).map_err(|e| error(e.to_string()))
    }
}

impl<N: Network> ToBytes for ArrayType<N> {
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        // Write the element type.
        self.element_type.write_le(&mut writer)?;
        // Write the number of dimensions of the array.
        self.num_dimensions.write_le(&mut writer)?;
        // Write the dimensions of the array,
        for i in 0..self.num_dimensions {
            self.dimensions[i as usize].write_le(&mut writer)?;
        }
        Ok(())
    }
}
