
fn rename [replacement: string] {
    let f = (ls | where type == "file" | select name)
    $f | each { |it|
        $it.name | str replace ($replacement) ""
    }
}