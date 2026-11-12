### The word's stalk and the task family: 78.179 renames (78.180 ruling 7, #165)

- **`ReceiptsEnvelope` becomes `WordStalk`**: the word's stalk, a sheaf over its star in the nerve. `Element::receipts()` and `set_receipts()` become `Element::stalk()` and `set_stalk()`.
- **`TaskEnvelope` becomes `TaskFamily`**: the task served as a family of manifolds (`ACT_TASK`), carrying train pairs and test inputs as samples of any rank.
- **Codecs & intake**: `encode_task_envelope` and `decode_task_envelope` become `encode_task` and `decode_task`; `PostedTask::envelope` becomes `PostedTask::family`; `Mind::perceive_envelope` becomes `Mind::perceive_task`.
