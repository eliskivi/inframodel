# inframodel

### Overview

Inframodel is a library for parsing InfraModel geotechnical ground investigation data formats commonly used in Finland. For the latest format specifications (version 2.5), please refer to the [Infra - Pohjatutkimusformaatti v2.5](https://sgy.fi/content/uploads/2018/11/infra_formaatti_v2-5_011118.pdf) document (available in Finnish).

### Key Types

- **InvestigationCollection**: A container for ground investigations sourced from multiple files. Each investigation within the collection may have different coordinate and elevation systems, as these details are stored within the `Investigation` type.

- **InfraFile**: Represents a single parsed InfraModel file. It contains a collection of ground investigations from that specific file. All investigations share the same coordinate and elevation systems, as these are stored within the `InfraFile` type.

- **Investigation**: Represents an individual ground investigation. It contains a collection of observations related to the investigation.

- **Observation**: Represents a single observation within an investigation.

- **ParseResult**: An enum representing the parsing status of a single value from the provided file. It can be:
    - `Parsed(T)`: Indicates that parsing was successful.
    - `Fallback(String)`: Indicates that parsing failed, providing a fallback string.
    - `None`: Indicates that the value is not present.

### Usage

Using InvestigationCollection as the container:

   ```rust
  use inframodel::*;

  let res = InvestigationCollection::parse_file("path/to/the/file.txt");
 
  let res = InvestigationCollection::parse_folder("path/to/the/folder"); 
   ```


Using InfraFile as the container:

   ```rust
  use inframodel::*;

  let res = InfraFile::parse_file("path/to/the/file.txt"); 
   ```
