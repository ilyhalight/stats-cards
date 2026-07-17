// https://huggingface.co/front/build/kube-0fe36e6/esm-RcS6-o5k.js <--
// ! u need to find actual script in sources (https://huggingface.co/models)
// ! that includes something like this `r={"text-classification":{name:`Text Classification`...`
const u = {};

const formatTagName = (name) => name.replaceAll(" ", "").replaceAll("-", "");

const formatLines = (lines, spaceCount, delimiter = ",\n") =>
  lines.map((line) => `${" ".repeat(spaceCount)}${line}`).join(delimiter);

const pipelineTagEnumItems = Object.keys(u).map((key) => {
  return [`#[serde(rename = "${key}")]`, `${formatTagName(u[key].name)}`];
});

const tagToStrList = Object.values(u).map((val) => {
  return `PipelineTag::${formatTagName(val.name)} => "${val.name}".to_string()`;
});

const pipelineTagEnumCode = `#[derive(Debug, Deserialize, Serialize)]
pub enum PipelineTag {
${pipelineTagEnumItems.map((item) => formatLines(item, 4, "\n")).join(",\n")}
}`;

const pipelineTagImplCode = `impl PipelineTag {
    pub fn to_string(&self) -> String {
        match self {
${formatLines(tagToStrList, 12)}
        }
    }
}`;

console.log([pipelineTagEnumCode, pipelineTagImplCode].join("\n\n"));
