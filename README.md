# Paska Bot
## Commands
### **~schedule**
#### <u>Arguments</u>
| Name | Type | Description | Optional | Comment |
| --- | --- | --- | --- | --- |
| name | string | Release Name | no | |
| date | date | Release Date | yes | can replace next |
| next | flag | Use next possible release date | yes | can replace date |
#### <u>Description</u>
Schedules a release at either the specified or the next date.
Returns a UUID in case rescheduling needs to be done
### **~nextDate**
#### <u>Arguments</u>
none
#### <u>Descriprion</u>
Returns the next possible release date
### **~remove**
#### <u>Arguments</u>
| Name | Type | Description | Optional | Comment |
| --- | --- | --- | --- | --- |
| name | string | Release name | yes | can replace uuid |
| uuid | string | Release UUID | yes | can replace name |
#### <u>Descriprion</u>
Removes a scheduled release
### **~reschedule**
#### <u>Arguments</u>
| Name | Type | Description | Optional | Comment |
| --- | --- | --- | --- | --- |
| name | string | Release name | yes | can replace uuid |
| uuid | string | Release UUID | yes | can replace name |
| newDate | date | new release date | yes | can replace next |
| next | flag | Use next possible release date | yes | can replace date |
#### <u>Descriprion</u>
Reschedules a release to a new date or the next possible date after the old date
### **~help**
#### <u>Arguments</u>
| Name | Type | Description | Optional | Comment |
| --- | --- | --- | --- | --- |
| command | string | command to paste help for | yes | |
#### <u>Descriprion</u>
Returns the README.md, optionally appending #(command) to skip to the specified command
### **~show**
#### <u>Arguments</u>
| Name | Type | Description | Optional | Comment |
| --- | --- | --- | --- | --- |
| date | Date | Date to show releases of | yes | can replace next |
| next | flag | Show next release | yes | can replace date |
#### <u>Descriprion</u>
Shows the releases of the specified date, or the next release
### **~showDate**
#### <u>Arguments</u>
| Name | Type | Description | Optional | Comment |
| --- | --- | --- | --- | --- |
| name | string | Release name | yes | can replace uuid |
| uuid | string | Release UUID | yes | can replace name |
#### <u>Descriprion</u>
Shows release date for the specified release
<!--
TEMPLATE
### **~**
#### <u>Arguments</u>
| Name | Type | Description | Optional | Comment |
| --- | --- | --- | --- | --- |
| | | | | |
#### <u>Descriprion</u>
-->