# Kubewarden policy fleet-image-override

## Description

This policy will override fleet images for testing custom fleet version in Rancher. This policy should not be used in Production!

## Settings

```
controller_image: raulcabm/fleet-controller:test1
```

## Example

```
apiVersion: policies.kubewarden.io/v1
kind: ClusterAdmissionPolicy
metadata:
  name: fleet-override
spec:
  module: ghcr.io/raulcabello/policies/fleet-image-override:latest
  rules:
  - apiGroups: ["", "apps", "batch"]
    apiVersions: ["v1"]
    resources: ["deployments"]
    operations:
    - CREATE
    - UPDATE
  mutating: true
  settings:
    controller_image: raulcabm/fleet:agent-fixv0.6.0
```

## License

```
Copyright (C) 2021 raul <raul.cabello@suse.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
