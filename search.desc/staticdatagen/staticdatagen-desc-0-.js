searchState.loadedDescShard("staticdatagen", 0, "Configuration errors\nContent processing errors\nContains the error value\nCustom error type for the shokunin library\nIO operation errors\nContains the success value\nError type for the shokunin library\nRepresents the Http Handle and its configuration.\nTemplate rendering errors\nVersion of the staticdatagen library.\nCompiler module for processing and generating static site …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nLocales module for language-specific translations and …\nChecks if a directory exists and creates it if necessary.\nCleans up (removes) multiple directories.\nCreates multiple directories at once.\nMacro for executing a shell command with logging\n<code>macro_get_args</code> Macro\nMacro for logging the completion of an operation\nMacro for logging an error\nCustom logging macro for various log levels and formats.\nMacro for logging the start of an operation\n<code>macro_metadata_option</code> Macro\n<code>macro_render_layout</code> Macro\n<code>macro_serve</code> Macro\nMacro definitions for common operations.\nData models and structures used throughout the crate.\nVarious modules for specific functionalities (e.g., HTML …\nCreates a new <code>Server</code> instance.\nStarts the server and begins listening for incoming …\nUtility functions and helpers.\nThe <code>service</code> module contains the compiler service. …\nCompiles source files in a specified directory into static …\nGerman language translations. Module for German …\nEnglish language translations.\nFrench language translations. Module for French …\nTemplate module for language-specific templates.\nTranslates the given text into German.\nTranslates the given text into English.\nTranslates the given text into French.\nThe <code>custom_macros</code> module contains all the custom macros …\nThe <code>directory_macros</code> module contains macros related to …\nThe <code>log_macros</code> module contains macros related to log …\nThe <code>shell_macros</code> module contains macros related to …\nErrors that can occur during command execution\nEncapsulates command execution functionality with safety …\nThe command string was empty\nThe command execution failed\nThe shell interpreter was not found\nThe command output could not be captured\nSets the shell command to execute\nExecutes the command and returns the result\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a new CommandExecutor instance with the specified …\nThe <code>data</code> module contains the structs. Core data models for …\nRepresents the CNAME data for a website\nContent exceeds maximum length\nErrors that can occur when working with data models\nRepresents the content and metadata of a file\nRepresents data for the humans.txt file\nRepresents data for an icon\nInvalid color code\nInvalid content\nInvalid date format\nInvalid domain name\nInvalid email format\nInvalid file name\nInvalid language code\nInvalid metadata\nInvalid size format\nInvalid Twitter handle\nInvalid URL format\nRepresents data for the web app manifest\nRepresents a single meta tag\nRepresents groups of meta tags for different platforms and …\nMissing required field\nRepresents data for the news sitemap\nRepresents options for the news sitemap visit function\nRepresents metadata for a single page\nRepresents data for the RSS feed\nRepresents data for the security.txt file according to RFC …\nSecurity-related validation error\nRepresents data for the service worker file\nRepresents tag metadata for pages\nRepresents data for the robots.txt file\nOptional: Link to a page where security researchers are …\nMeta tags specific to Apple devices\nThe Atom link of the RSS feed\nThe name of the author\nThe author of the RSS feed\nThe location of the author\nThe Twitter handle of the author\nThe website of the author\nThe background color of the web app\nThe base URL of the news website\nOptional: Canonical URI where this security.txt file is …\nThe category of the RSS feed\nThe domain name for the website\nThe CNAME content, if applicable\nRequired: One or more URIs or email addresses for …\nThe main content of the file\nThe copyright notice for the content of the feed\nCreates a new <code>NewsData</code> instance with default values\nCreates a new SecurityData instance with all fields empty\nThe publication date of the page\nPublication dates for tagged pages\nA brief description of the page content\nA description of the web app\nThe description of the RSS feed\nDescriptions of tagged pages\nThe display mode of the web app\nThe documentation URL for the RSS feed format\nOptional: Link to an encryption key\nRequired: Expiration date for the security.txt data (in …\nReturns the file extension\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGenerates the HTML representation of the meta tag\nGenerates the robots.txt content\nGenerates a complete list of metatags in HTML format\nThe generator of the RSS feed\nReturns the genres as a vector\nReturns the value for the given key, if it exists\nReturns a list of all non-empty fields\nOptional: Link to security-related job positions\nThe human-readable metadata\nThe MIME type of the icon\nIcons associated with the web app\nThe image URL for the RSS feed\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns true if all fields are empty\nReturns true if the file is a markdown file\nReturns true if all optional fields are empty\nValidates if the required fields are properly set\nThe description of an RSS item\nThe unique identifier (GUID) of an RSS item\nThe link to an RSS item\nThe publication date of an RSS item\nThe title of an RSS item\nThe JSON representation of the file content\nKeywords associated with the file content\nKeywords associated with tagged pages\nReturns keywords as a vector\nThe language of the RSS feed\nThe last build date of the RSS feed\nThe link to the website associated with the RSS feed\nThe managing editor of the RSS feed\nMicrosoft-specific meta tags\nThe name of the file\nThe name of the web app\nThe name of the meta tag\nCreates a new <code>CnameData</code> instance\nCreates a new PageData instance\nCreates a new <code>FileData</code> instance\nCreates a new <code>TagsData</code> instance\nCreates a new <code>SwFileData</code> instance\nCreates a new <code>IconData</code> instance\nCreates a new <code>ManifestData</code> instance with default values\nCreates a new <code>NewsData</code> instance\nCreates a new <code>NewsVisitOptions</code> instance\nCreates a new <code>HumansData</code> instance\nCreates a new <code>MetaTagGroups</code> instance with default values\nCreates a new <code>TxtData</code> instance\nCreates a new <code>RssData</code> instance with default values\nCreates a new <code>MetaTag</code> instance\nCreates a new SecurityData instance with required fields\nThe genres of the news content\nThe genres of the news content\nThe URL of the news image\nKeywords associated with the news content\nKeywords associated with the news content\nThe language of the news content\nThe language of the news content\nThe URL of the news content\nThe publication date of the news content\nThe publication date of the news content\nThe name of the news publication\nThe name of the news publication\nThe title of the news content\nThe title of the news content\nURL of the offline page\nOpen Graph meta tags, mainly used for social media\nThe orientation of the web app\nThe permanent link to the page\nThe permalink of the website\nPermalinks to tagged pages\nOptional: Link to the security policy\nOptional: Preferred languages for security reports …\nPrimary meta tags, such as author, description, etc.\nThe publication date of the RSS feed\nThe purpose of the icon (e.g., “maskable”, “any”)\nThe RSS feed content\nReturns a sanitized version of the title\nThe scope of the web app\nThe security.txt content\nSets the value of a field\nThe short name of the web app\nThe components used in the site\nThe date when the site was last updated\nThe software used to build the site\nThe standards followed by the site\nThe sitemap content\nThe news sitemap content\nThe sizes of the icon (e.g., “192x192”)\nThe source URL of the icon\nThe start URL of the web app\nAcknowledgements or thanks\nThe theme color of the web app\nThe title of the page\nThe title of the RSS feed\nTitles of tagged pages\nTime To Live: the number of minutes the feed should be …\nTwitter-specific meta tags\nThe robots.txt content\nValidates the CNAME data\nValidates the page data\nValidates the file data\nValidates the tags data\nValidates the service worker data\nValidates the icon data\nValidates the manifest data\nValidates the news data\nValidates the news visit options\nValidates the humans.txt data\nValidates meta tag content\nValidates the robots.txt data\nValidates the RSS data\nValidates the meta tag\nValidates the security.txt data according to RFC 9116\nCommon validation functions for data models\nThe content of the meta tag\nThe webmaster of the RSS feed\nSanitizes a file path\nValidates color format (hex or RGB)\nValidates a date string in RFC3339 format\nValidates image size format (e.g., “192x192”)\nValidates language code against ISO 639-1\nValidates text length against a maximum limit\nValidates Twitter handle format\nValidates a URL string\nThe <code>cname</code> module generates the CNAME content. CNAME Record …\nThe <code>human</code> module contains functions for generating …\nThe <code>json</code> module generates the JSON content. JSON and data …\nThe <code>manifest</code> module generates the manifest file. Web App …\nThe <code>navigation</code> module generates the navigation menu. …\nThe <code>newssitemap</code> module generates the newssitemap content. …\nThe <code>plaintext</code> module contains functions for generating …\nthe <code>postprocessor</code> module contains functions for …\nThe <code>preprocessor</code> module contains functions for …\nThe <code>robots</code> module generates the robots.txt content. …\nThe <code>security</code> module generates the security.txt content. …\nThe <code>tags</code> module contains functions for generating a tags …\nCreates a CnameData object from metadata.\nGenerates CNAME record content.\nCreates a HumansData object from metadata.\nGenerates humans.txt content.\nGenerates CNAME file content.\nGenerates a single news sitemap entry\nGenerates humans.txt file content.\nGenerates web app manifest content.\nGenerates a news sitemap in XML format.\nGenerates security.txt file content according to RFC 9116.\nGenerates a sitemap based on provided configuration\nGenerates robots.txt content\nCreates a ManifestData object from metadata.\nNavigation menu generator\nReturns the argument unchanged.\nGenerates a navigation menu as an unordered list of links.\nCalls <code>U::from(self)</code>.\nConverts date strings from “Tue, 20 Feb 2024 15:15:15 GMT…\nCreates a NewsData object from metadata.\nInvalid configuration error\nContent length exceeds maximum limits\nParsing error during content conversion\nConfiguration options for plain text generation\nErrors that can occur during plain text generation\nUnicode validation error\nWhether to use ASCII-only output\nReturns the argument unchanged.\nReturns the argument unchanged.\nGenerates plain text content from HTML/Markdown input.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nList item bullet character\nMaximum line length for wrapping\nWhether to preserve empty lines between sections\nCustom text replacements\nPost-processes HTML content by performing various …\nPreprocesses the Markdown content to update class …\nCreates a TxtData object from metadata.\nGenerates robots.txt content.\nCreates a SecurityData object from metadata.\nGenerates security.txt content.\nCreates a <code>TagsData</code> struct populated with metadata.\nGenerates a tag list from the given <code>FileData</code> and metadata, …\nGenerates the HTML content for displaying tags and their …\nWrites the given HTML content into an existing <code>index.html</code> …\nThe <code>backup</code> module contains functions for creating backups …\nThe <code>directory</code> module contains functions for creating …\nThe <code>element</code> module contains functions for writing XML …\nThe <code>file</code> module handles file reading and writing …\nThe <code>security</code> module contains functions for …\nThe <code>uuid</code> module contains functions for generating unique …\nThe <code>write</code> module contains functions for writing files. …\nCreates a backup of a file.\nCleans up the specified directories.\nCreates and returns a <code>comrak::ComrakOptions</code> instance with …\nCreates new directories at the specified paths.\nEnsures a directory exists, creating it if necessary.\nExtracts the front matter from the given content.\nFinds all HTML files in a directory and its subdirectories.\nFormats a header string with an ID and class attribute.\nMoves the output directory to the public directory.\nConverts a string to title case.\nTruncates a path to only have a set number of path …\nUpdates the ‘class’ attributes within the provided …\nHelper function to write XML element\nReads all files in a directory specified by the given path …\nSanitizes a file path to prevent path traversal attacks.\nValidates a directory for security and accessibility.\nGenerates a unique string.\nWrites the files to the build directory.")