use crate::ut;

/// Abstract base types
impl ut::Type {
    /// A generic base type for most things (files, directories.)
    ///
    /// UTI: public.item
    pub fn item() -> &'static Self {
        unsafe { UTTypeItem }
    }

    /// A base type for anything containing user-viewable document content
    /// (documents, pasteboard data, and document packages.)
    ///
    /// UTI: public.content
    pub fn content() -> &'static Self {
        unsafe { UTTypeContent }
    }

    /// A base type for content formats supporting mixed embedded content
    /// (i.e., compound documents).
    ///
    /// UTI: public.composite-content
    /// conforms to: public.content
    pub fn composite_content() -> &'static Self {
        unsafe { UTTypeCompositeContent }
    }

    /// A data item mountable as a volume
    ///
    /// UTI: public.disk-image
    pub fn disk_image() -> &'static Self {
        unsafe { UTTypeDiskImage }
    }
}
#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeItem: &'static ut::Type;
    static UTTypeContent: &'static ut::Type;
    static UTTypeCompositeContent: &'static ut::Type;
    static UTTypeDiskImage: &'static ut::Type;
}

/// Concrete base types
impl ut::Type {
    /// A base type for any sort of simple byte stream, including files and
    /// in-memory data.
    ///
    /// UTI: public.data
    /// conforms to: public.item
    pub fn data() -> &'static Self {
        unsafe { UTTypeData }
    }

    /// A file system directory (includes packages \em and folders.)
    ///
    /// UTI: public.directory
    /// conforms to: public.item
    pub fn directory() -> &'static Self {
        unsafe { UTTypeDirectory }
    }

    /// Symbolic link and alias file types conform to this type.
    ///
    /// UTI: com.apple.resolvable
    pub fn resolvable() -> &'static Self {
        unsafe { UTTypeResolvable }
    }

    /// A symbolic link.
    ///
    /// UTI: public.symlink
    /// conforms to: public.item, com.apple.resolvable
    pub fn symbolic_link() -> &'static Self {
        unsafe { UTTypeSymbolicLink }
    }

    /// An executable item.
    ///
    /// UTI: public.executable
    /// conforms to: public.item
    pub fn executable() -> &'static Self {
        unsafe { UTTypeExecutable }
    }

    /// A volume mount point (resolvable, resolves to the root directory of a
    /// volume.)
    ///
    /// UTI: com.apple.mount-point
    /// conforms to: public.item, com.apple.resolvable
    pub fn mount_point() -> &'static Self {
        unsafe { UTTypeMountPoint }
    }

    /// A fully-formed alias file.
    ///
    /// UTI: com.apple.alias-file
    /// conforms to: public.data, com.apple.resolvable
    pub fn alias_file() -> &'static Self {
        unsafe { UTTypeAliasFile }
    }
}
#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeData: &'static ut::Type;
    static UTTypeDirectory: &'static ut::Type;
    static UTTypeResolvable: &'static ut::Type;
    static UTTypeSymbolicLink: &'static ut::Type;
    static UTTypeExecutable: &'static ut::Type;
    static UTTypeMountPoint: &'static ut::Type;
    static UTTypeAliasFile: &'static ut::Type;
    static UTTypeURLBookmarkData: &'static ut::Type;
}

/// URL types
impl ut::Type {
    /// A URL bookmark.
    ///
    /// UTI: com.apple.bookmark
    /// conforms to: public.data, com.apple.resolvable
    pub fn url_bookmark_data() -> &'static Self {
        unsafe { UTTypeURLBookmarkData }
    }

    /// Any URL.
    ///
    /// UTI: public.url
    /// conforms to: public.data
    pub fn url() -> &'static Self {
        unsafe { UTTypeURL }
    }

    /// A URL with the scheme \c "file:".
    ///
    /// UTI: public.file-url
    /// conforms to: public.url
    pub fn file_url() -> &'static Self {
        unsafe { UTTypeFileURL }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeURL: &'static ut::Type;
    static UTTypeFileURL: &'static ut::Type;
}

/// Text types
impl ut::Type {
    /// The base type for all text-encoded data, including text with markup
    /// (HTML, RTF, etc.).
    ///
    /// UTI: public.text
    /// conforms to: public.data, public.content
    pub fn text() -> &'static Self {
        unsafe { UTTypeText }
    }

    /// Text with no markup and an unspecified encoding.
    ///
    /// UTI: public.plain-text
    /// conforms to: public.text
    pub fn plain_text() -> &'static Self {
        unsafe { UTTypePlainText }
    }

    /// Plain text encoded as UTF-8.
    ///
    /// UTI: public.utf8-plain-text
    /// conforms to: public.plain-text
    pub fn utf8_plain_text() -> &'static Self {
        unsafe { UTTypeUTF8PlainText }
    }

    /// Plain text encoded as UTF-16 with a BOM, or if a BOM is not present,
    /// using "external representation" byte order (big-endian).
    ///
    /// UTI: public.utf16-external-plain-text
    /// conforms to: public.plain-text
    pub fn utf16_external_plain_text() -> &'static Self {
        unsafe { UTTypeUTF16ExternalPlainText }
    }

    /// Plain text encoded as UTF-16, in native byte order, with an optional
    /// BOM.
    ///
    /// UTI: public.utf16-plain-text
    /// conforms to: public.plain-text
    pub fn utf16_plain_text() -> &'static Self {
        unsafe { UTTypeUTF16PlainText }
    }

    /// Text containing delimited values.
    ///
    /// UTI: public.delimited-values-text
    /// conforms to: public.text
    pub fn delimited_text() -> &'static Self {
        unsafe { UTTypeDelimitedText }
    }

    /// Text containing comma-separated values (.csv).
    ///
    /// UTI: public.comma-separated-values-text
    /// conforms to: public.delimited-values-text
    pub fn comma_separated_text() -> &'static Self {
        unsafe { UTTypeCommaSeparatedText }
    }

    /// Text containing tab-separated values.
    ///
    /// UTI: public.tab-separated-values-text
    /// conforms to: public.delimited-values-text
    pub fn tab_separated_text() -> &'static Self {
        unsafe { UTTypeTabSeparatedText }
    }

    /// UTF-8 encoded text containing tab-separated values.
    ///
    /// UTI: public.utf8-tab-separated-values-text
    /// conforms to: public.tab-separated-values-text, public.utf8-plain-text
    pub fn utf8_tab_separated_text() -> &'static Self {
        unsafe { UTTypeUTF8TabSeparatedText }
    }

    /// Rich Text Format data.
    ///
    /// UTI: public.rtf
    /// conforms to: public.text
    pub fn rtf() -> &'static Self {
        unsafe { UTTypeRTF }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeText: &'static ut::Type;
    static UTTypePlainText: &'static ut::Type;
    static UTTypeUTF8PlainText: &'static ut::Type;
    static UTTypeUTF16ExternalPlainText: &'static ut::Type;
    static UTTypeUTF16PlainText: &'static ut::Type;
    static UTTypeDelimitedText: &'static ut::Type;
    static UTTypeCommaSeparatedText: &'static ut::Type;
    static UTTypeTabSeparatedText: &'static ut::Type;
    static UTTypeUTF8TabSeparatedText: &'static ut::Type;
    static UTTypeRTF: &'static ut::Type;
}

/// Markup languages
impl ut::Type {
    /// Any version of HTML.
    ///
    /// UTI: public.html
    /// conforms to: public.text
    pub fn html() -> &'static Self {
        unsafe { UTTypeHTML }
    }

    /// Generic XML.
    ///
    /// UTI: public.xml
    /// conforms to: public.text
    pub fn xml() -> &'static Self {
        unsafe { UTTypeXML }
    }

    /// Yet Another Markup Language.
    ///
    /// UTI: public.yaml
    /// conforms to: public.text
    pub fn yaml() -> &'static Self {
        unsafe { UTTypeYAML }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeHTML: &'static ut::Type;
    static UTTypeXML: &'static ut::Type;
    static UTTypeYAML: &'static ut::Type;
}

/// Programming languages
impl ut::Type {
    /// Abstract type for source code of any language.
    ///
    /// UTI: public.source-code
    /// conforms to: public.plain-text
    pub fn src_code() -> &'static Self {
        unsafe { UTTypeSourceCode }
    }

    /// Assembly language source (.s)
    ///
    /// UTI: public.assembly-source
    /// conforms to: public.source-code
    pub fn asm_src() -> &'static Self {
        unsafe { UTTypeAssemblyLanguageSource }
    }

    /// C source code (.c)
    ///
    /// UTI: public.c-source
    /// conforms to: public.source-code
    pub fn c_src() -> &'static Self {
        unsafe { UTTypeCSource }
    }

    /// Objective-C source code (.m)
    ///
    /// UTI: public.objective-c-source
    /// conforms to: public.source-code
    pub fn obj_c_src() -> &'static Self {
        unsafe { UTTypeObjectiveCSource }
    }

    /// Swift source code (.swift)
    ///
    /// UTI: public.swift-source
    /// conforms to: public.source-code
    pub fn swift_src() -> &'static Self {
        unsafe { UTTypeSwiftSource }
    }

    /// C++ source code (.cp, etc.)
    ///
    /// UTI: public.c-plus-plus-source
    /// conforms to: public.source-code
    pub fn cpp_src() -> &'static Self {
        unsafe { UTTypeCPlusPlusSource }
    }
    /// Objective-C++ source code.
    ///
    /// UTI: public.objective-c-plus-plus-source
    /// conforms to: public.source-code
    pub fn obj_cpp_src() -> &'static Self {
        unsafe { UTTypeObjectiveCPlusPlusSource }
    }

    /// A C header.
    ///
    /// UTI: public.c-header
    /// conforms to: public.source-code
    pub fn c_header() -> &'static Self {
        unsafe { UTTypeCHeader }
    }

    /// A C++ header.
    ///
    /// UTI: public.c-plus-plus-header
    /// conforms to: public.source-code
    pub fn cpp_header() -> &'static Self {
        unsafe { UTTypeCPlusPlusHeader }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeSourceCode: &'static ut::Type;
    static UTTypeAssemblyLanguageSource: &'static ut::Type;
    static UTTypeCSource: &'static ut::Type;
    static UTTypeObjectiveCSource: &'static ut::Type;
    static UTTypeSwiftSource: &'static ut::Type;
    static UTTypeCPlusPlusSource: &'static ut::Type;
    static UTTypeObjectiveCPlusPlusSource: &'static ut::Type;
    static UTTypeCHeader: &'static ut::Type;
    static UTTypeCPlusPlusHeader: &'static ut::Type;
}

/// Scripting languages
impl ut::Type {
    /// A base type for any scripting language source.
    ///
    /// UTI: public.script
    /// conforms to: public.source-code
    pub fn script() -> &'static Self {
        unsafe { UTTypeScript }
    }

    /// An AppleScript text-based script (.applescript).
    ///
    /// UTI: com.apple.applescript.text
    /// conforms to: public.script
    pub fn apple_script() -> &'static Self {
        unsafe { UTTypeAppleScript }
    }

    /// An Open Scripting Architecture binary script (.scpt).
    ///
    /// UTI: com.apple.applescript.script
    /// conforms to: public.data, public.script
    pub fn osa_script() -> &'static Self {
        unsafe { UTTypeOSAScript }
    }

    /// An Open Scripting Architecture script bundle (.scptd).
    ///
    /// UTI: com.apple.applescript.script-bundle
    /// conforms to: com.apple.bundle, com.apple.package, public.script
    pub fn osa_script_bundle() -> &'static Self {
        unsafe { UTTypeOSAScriptBundle }
    }

    /// JavaScript source code
    ///
    /// UTI: com.netscape.javascript-source
    /// conforms to: public.source-code, public.executable
    pub fn java_script() -> &'static Self {
        unsafe { UTTypeJavaScript }
    }

    /// The base type for shell scripts.
    ///
    /// UTI: public.shell-script
    /// conforms to: public.script
    pub fn shell_script() -> &'static Self {
        unsafe { UTTypeShellScript }
    }

    /// A Perl script.
    ///
    /// UTI: public.perl-script
    /// conforms to: public.shell-script
    pub fn perl_script() -> &'static Self {
        unsafe { UTTypePerlScript }
    }

    /// A Python script.
    ///
    /// UTI: public.python-script
    /// conforms to: public.shell-script
    pub fn python_script() -> &'static Self {
        unsafe { UTTypePythonScript }
    }

    /// A Ruby script.
    ///
    /// UTI: public.ruby-script
    /// conforms to: public.shell-script
    pub fn ruby_script() -> &'static Self {
        unsafe { UTTypeRubyScript }
    }

    /// A PHP script.
    ///
    /// UTI: public.php-script
    /// conforms to: public.shell-script
    pub fn php_script() -> &'static Self {
        unsafe { UTTypePHPScript }
    }

    /// A makefile.
    ///
    /// UTI: public.make-source
    /// conforms to: public.script
    pub fn makefile() -> &'static Self {
        unsafe { UTTypeMakefile }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeScript: &'static ut::Type;
    static UTTypeAppleScript: &'static ut::Type;
    static UTTypeOSAScript: &'static ut::Type;
    static UTTypeOSAScriptBundle: &'static ut::Type;
    static UTTypeJavaScript: &'static ut::Type;
    static UTTypeShellScript: &'static ut::Type;
    static UTTypePerlScript: &'static ut::Type;
    static UTTypePythonScript: &'static ut::Type;
    static UTTypeRubyScript: &'static ut::Type;
    static UTTypePHPScript: &'static ut::Type;
    static UTTypeMakefile: &'static ut::Type;
}

/// Serialized data types
impl ut::Type {
    /// JavaScript object notation (JSON) data
    ///
    /// UTI: public.json
    /// conforms to: public.text
    /// JSON almost (but doesn't quite) conforms to
    /// com.netscape.javascript-source.
    pub fn json() -> &'static Self {
        unsafe { UTTypeJSON }
    }

    /// A base type for property lists.
    ///
    /// UTI: com.apple.property-list
    /// conforms to: public.data
    pub fn prop_list() -> &'static Self {
        unsafe { UTTypePropertyList }
    }

    /// An XML property list.
    ///
    /// UTI: com.apple.xml-property-list
    /// conforms to: public.xml, com.apple.property-list
    pub fn xml_prop_list() -> &'static Self {
        unsafe { UTTypeXMLPropertyList }
    }

    /// A binary property list.
    ///
    /// UTI: com.apple.binary-property-list
    /// conforms to: com.apple.property-list
    pub fn binary_prop_list() -> &'static Self {
        unsafe { UTTypeBinaryPropertyList }
    }
}
#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeJSON: &'static ut::Type;
    static UTTypePropertyList: &'static ut::Type;
    static UTTypeXMLPropertyList: &'static ut::Type;
    static UTTypeBinaryPropertyList: &'static ut::Type;
}
