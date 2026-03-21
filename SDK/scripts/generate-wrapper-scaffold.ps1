Param(
    [string]$CapabilitiesPath = "spec/capabilities.json",
    [string]$WrapperDir = "src/wrapper"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function To-SnakeCase([string]$name) {
    if ([string]::IsNullOrEmpty($name)) {
        return $name
    }

    $withUnderscore = [regex]::Replace($name, "([a-z0-9])([A-Z])", '$1_$2')
    return $withUnderscore.ToLowerInvariant()
}

$caps = Get-Content $CapabilitiesPath -Raw | ConvertFrom-Json

$resourceConfig = @{
    user = @{
        api = "UserApi"
        options = "UserOptions"
        model_module = "user"
        model_type = "UserProfileBundle"
        typed_method = "profile"
        typed_selection = "profile"
        typed_mode = "user_id"
    }
    faction = @{
        api = "FactionApi"
        options = "FactionOptions"
        model_module = "faction"
        model_type = "FactionBasicBundle"
        typed_method = "basic"
        typed_selection = "basic"
        typed_mode = "options"
    }
    forum = @{
        api = "ForumApi"
        options = "ForumOptions"
        model_module = "forum"
        model_type = "ForumThreadBundle"
        typed_method = "thread"
        typed_selection = "thread"
        typed_mode = "options"
    }
    key = @{
        api = "KeyApi"
        options = "KeyOptions"
        model_module = "key"
        model_type = "KeyInfoBundle"
        typed_method = "info"
        typed_selection = "info"
        typed_mode = "options"
    }
    market = @{
        api = "MarketApi"
        options = "MarketOptions"
        model_module = "market"
        model_type = "MarketBazaarBundle"
        typed_method = "bazaar"
        typed_selection = "bazaar"
        typed_mode = "options"
    }
    property = @{
        api = "PropertyApi"
        options = "PropertyOptions"
        model_module = "property"
        model_type = "PropertyBundle"
        typed_method = "property"
        typed_selection = "property"
        typed_mode = "options"
    }
    racing = @{
        api = "RacingApi"
        options = "RacingOptions"
        model_module = "racing"
        model_type = "RacingRaceBundle"
        typed_method = "race"
        typed_selection = "race"
        typed_mode = "options"
    }
    torn = @{
        api = "TornApi"
        options = "TornOptions"
        model_module = "torn"
        model_type = "TornCalendarBundle"
        typed_method = "calendar"
        typed_selection = "calendar"
        typed_mode = "options"
    }
}

foreach ($resourceName in $resourceConfig.Keys) {
    $cfg = $resourceConfig[$resourceName]
    $resource = $caps.resources.$resourceName
    if ($null -eq $resource) {
        throw "Resource '${resourceName}' not found in ${CapabilitiesPath}"
    }

    $selectionNames = @($resource.selections | ForEach-Object { $_.name })

    $pathParamNames = @{}
    foreach ($selection in $resource.selections) {
        foreach ($param in @($selection.required_path_parameters)) {
            if ([string]::IsNullOrWhiteSpace($param)) {
                continue
            }
            if ($param -eq "id") {
                continue
            }
            $pathParamNames[$param] = $true
        }
    }
    $pathParams = @($pathParamNames.Keys | Sort-Object)

    $apiName = $cfg.api
    $optionsName = $cfg.options
    $modelModule = $cfg.model_module
    $modelType = $cfg.model_type
    $typedMethod = $cfg.typed_method
    $typedSelection = $cfg.typed_selection
    $typedMode = $cfg.typed_mode

    $lines = New-Object System.Collections.Generic.List[string]
    $lines.Add("use crate::client::{DataRequestOptions, TornClient};")
    $lines.Add("use crate::models::generated::RawSelectionBundle;")
    $lines.Add("use crate::models::manual::${modelModule}::${modelType};")
    $lines.Add("use crate::transport::HttpTransport;")
    $lines.Add("use crate::wrapper::error::SdkError;")
    $lines.Add(
        "use crate::wrapper::internal::{execute_raw, execute_typed, validate_selection, validate_selections};"
    )
    $lines.Add("use crate::wrapper::options::BaseOptions;")
    $lines.Add("")
    $lines.Add("macro_rules! raw_selection_methods {")
    $lines.Add("    (`$options:ty; `$(`$method:ident => `$selection:literal),* `$(,)?) => {")
    $lines.Add("        `$(")
    $lines.Add(
        "            pub async fn `$method(&self, options: `$options) -> Result<RawSelectionBundle, SdkError> {"
    )
    $lines.Add("                self.raw_selection(`$selection, options).await")
    $lines.Add("            }")
    $lines.Add("        )*")
    $lines.Add("    };")
    $lines.Add("}")
    $lines.Add("")

    $lines.Add("#[derive(Debug, Clone, Default)]")
    $lines.Add("pub struct ${optionsName} {")
    $lines.Add("    pub base: BaseOptions,")
    foreach ($param in $pathParams) {
        $field = To-SnakeCase $param
        $lines.Add("    pub ${field}: Option<String>,")
    }
    $lines.Add("}")
    $lines.Add("")
    $lines.Add("impl ${optionsName} {")
    $lines.Add("    pub fn with_base(mut self, base: BaseOptions) -> Self {")
    $lines.Add("        self.base = base;")
    $lines.Add("        self")
    $lines.Add("    }")
    $lines.Add("")
    $lines.Add("    pub fn with_id(mut self, id: impl Into<String>) -> Self {")
    $lines.Add("        self.base = self.base.with_id(id);")
    $lines.Add("        self")
    $lines.Add("    }")
    foreach ($param in $pathParams) {
        $field = To-SnakeCase $param
        $lines.Add("")
        $lines.Add("    pub fn with_${field}(mut self, value: impl Into<String>) -> Self {")
        $lines.Add("        self.${field} = Some(value.into());")
        $lines.Add("        self")
        $lines.Add("    }")
    }
    $lines.Add("")
    $lines.Add("    pub(crate) fn into_data_request_options(self) -> DataRequestOptions {")
    if ($pathParams.Count -eq 0) {
        $lines.Add("        self.base.into_data_request_options()")
    } else {
        $lines.Add("        let mut options = self.base.into_data_request_options();")
        foreach ($param in $pathParams) {
            $field = To-SnakeCase $param
            $lines.Add("        if let Some(value) = self.${field} {")
            $lines.Add("            options = options.with_path_arg(`"${param}`", value);")
            $lines.Add("        }")
        }
        $lines.Add("        options")
    }
    $lines.Add("    }")
    $lines.Add("}")
    $lines.Add("")

    $lines.Add("#[derive(Debug, Clone, Copy)]")
    $lines.Add("pub struct ${apiName}<'a, T: HttpTransport> {")
    $lines.Add("    pub(crate) client: &'a TornClient<T>,")
    $lines.Add("}")
    $lines.Add("")
    $lines.Add("impl<'a, T: HttpTransport> ${apiName}<'a, T> {")
    $lines.Add("    pub const SUPPORTED_SELECTIONS: &'static [&'static str] = &[")
    foreach ($selection in $selectionNames) {
        $lines.Add("        `"${selection}`",")
    }
    $lines.Add("    ];")
    $lines.Add("")
    $lines.Add("    pub fn raw_client(&self) -> &TornClient<T> {")
    $lines.Add("        self.client")
    $lines.Add("    }")
    $lines.Add("")

    if ($typedMode -eq "user_id") {
        $lines.Add("    pub async fn ${typedMethod}(")
        $lines.Add("        &self,")
        $lines.Add("        user_id: impl Into<String>,")
        $lines.Add("    ) -> Result<${modelType}, SdkError> {")
        $lines.Add("        let options = ${optionsName}::default().with_id(user_id.into());")
        $lines.Add("        execute_typed(")
        $lines.Add("            self.client,")
        $lines.Add("            `"${resourceName}`",")
        $lines.Add("            vec![`"${typedSelection}`".to_string()],")
        $lines.Add("            options.into_data_request_options(),")
        $lines.Add("        )")
        $lines.Add("        .await")
        $lines.Add("    }")
    } else {
        $lines.Add("    pub async fn ${typedMethod}(")
        $lines.Add("        &self,")
        $lines.Add("        options: ${optionsName},")
        $lines.Add("    ) -> Result<${modelType}, SdkError> {")
        $lines.Add("        execute_typed(")
        $lines.Add("            self.client,")
        $lines.Add("            `"${resourceName}`",")
        $lines.Add("            vec![`"${typedSelection}`".to_string()],")
        $lines.Add("            options.into_data_request_options(),")
        $lines.Add("        )")
        $lines.Add("        .await")
        $lines.Add("    }")
    }

    $lines.Add("")
    $lines.Add("    pub async fn raw_selection(")
    $lines.Add("        &self,")
    $lines.Add("        selection: &str,")
    $lines.Add("        options: ${optionsName},")
    $lines.Add("    ) -> Result<RawSelectionBundle, SdkError> {")
    $lines.Add(
        "        validate_selection(`"${resourceName}`", selection, Self::SUPPORTED_SELECTIONS)?;"
    )
    $lines.Add("        execute_raw(")
    $lines.Add("            self.client,")
    $lines.Add("            `"${resourceName}`",")
    $lines.Add("            vec![selection.to_string()],")
    $lines.Add("            options.into_data_request_options(),")
    $lines.Add("        )")
    $lines.Add("        .await")
    $lines.Add("    }")
    $lines.Add("")

    $lines.Add("    pub async fn raw_selections<S, I>(")
    $lines.Add("        &self,")
    $lines.Add("        selections: I,")
    $lines.Add("        options: ${optionsName},")
    $lines.Add("    ) -> Result<RawSelectionBundle, SdkError>")
    $lines.Add("    where")
    $lines.Add("        S: Into<String>,")
    $lines.Add("        I: IntoIterator<Item = S>,")
    $lines.Add("    {")
    $lines.Add("        let selection_values = selections.into_iter().map(Into::into).collect::<Vec<_>>();")
    $lines.Add(
        "        validate_selections(`"${resourceName}`", &selection_values, Self::SUPPORTED_SELECTIONS)?;"
    )
    $lines.Add("        execute_raw(")
    $lines.Add("            self.client,")
    $lines.Add("            `"${resourceName}`",")
    $lines.Add("            selection_values,")
    $lines.Add("            options.into_data_request_options(),")
    $lines.Add("        )")
    $lines.Add("        .await")
    $lines.Add("    }")
    $lines.Add("")

    $lines.Add("    pub async fn typed_selection<R>(")
    $lines.Add("        &self,")
    $lines.Add("        selection: &str,")
    $lines.Add("        options: ${optionsName},")
    $lines.Add("    ) -> Result<R, SdkError>")
    $lines.Add("    where")
    $lines.Add("        R: serde::de::DeserializeOwned,")
    $lines.Add("    {")
    $lines.Add(
        "        validate_selection(`"${resourceName}`", selection, Self::SUPPORTED_SELECTIONS)?;"
    )
    $lines.Add("        execute_typed(")
    $lines.Add("            self.client,")
    $lines.Add("            `"${resourceName}`",")
    $lines.Add("            vec![selection.to_string()],")
    $lines.Add("            options.into_data_request_options(),")
    $lines.Add("        )")
    $lines.Add("        .await")
    $lines.Add("    }")
    $lines.Add("")

    $lines.Add("    raw_selection_methods!(${optionsName};")
    foreach ($selection in $selectionNames) {
        $methodName = "${selection}_raw"
        $lines.Add("        ${methodName} => `"${selection}`",")
    }
    $lines.Add("    );")
    $lines.Add("}")

    $targetPath = Join-Path $WrapperDir "${resourceName}.rs"
    Set-Content -Path $targetPath -Value ($lines -join "`n") -NoNewline
    Write-Host "Generated ${targetPath}"
}
