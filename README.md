# inframodel

### Overview

Inframodel is a library for parsing InfraModel geotechnical ground investigation data formats commonly used in Finland. For the latest format specifications (version 2.5), please refer to the [Infra - Pohjatutkimusformaatti v2.5](https://sgy.fi/content/uploads/2018/11/infra_formaatti_v2-5_011118.pdf) document (available in Finnish).

### Key types

- **InfraFile**: Represents a single parsed InfraModel file. Contains a collection of ground investigations contained within the parsed file.

- **Investigation**: Represents an individual ground investigation. Containts a collection of observations related to the investigation.

- **Observation**: Represents a single observation within an investigation.

### Usage

   ```rust
  use inframodel::*;

  let parsed_infra_file = InfraFile::parse_file("path/to/the/file.txt");
   ```
