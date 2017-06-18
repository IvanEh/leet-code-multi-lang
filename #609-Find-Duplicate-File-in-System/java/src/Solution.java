import java.util.*;
import java.util.stream.*;

public class Solution {
    private static int KEY_LEN_THRESHOLD = 16;

    private Map<String, List<FileEntry>> fileEntries = new HashMap<>();

    public List<List<String>> findDuplicate(String[] paths) {
        storePaths(paths);
        List<List<FileEntry>> duplicates = extractDuplicates();
        return duplicates.stream()
            .map(lst -> lst.stream().map(FileEntry::toString).collect(Collectors.toList()))
            .collect(Collectors.toList());
    }

    private List<List<FileEntry>> extractDuplicates() {
        List<List<FileEntry>> duplicateGroups = new ArrayList<>();

        for (Map.Entry<String, List<FileEntry>> entry: fileEntries.entrySet()) {
            List<List<FileEntry>> currentDuplicateGroups = extractDuplicatesInsideMapEntry(entry);
            duplicateGroups.addAll(currentDuplicateGroups);
        }

        return duplicateGroups;
    }

    private List<List<FileEntry>> extractDuplicatesInsideMapEntry(Map.Entry<String, List<FileEntry>> entry) {
        List<FileEntry> fileEntries = entry.getValue() != null ? entry.getValue() : new ArrayList<>();
        List<List<FileEntry>> duplicateGroups = new ArrayList<>();

        for(int i = 0; i < fileEntries.size(); i++) {
            List<FileEntry> duplicateGroup = new ArrayList<>();
            FileEntry pivot = fileEntries.get(i);
            duplicateGroup.add(pivot);
            
            for (int j = i + 1; j < fileEntries.size(); j++) {
                FileEntry fileEntry = fileEntries.get(j);
                
                if (fileEntry.content.equals(pivot.content)) {
                    duplicateGroup.add(fileEntry);
                    fileEntries.remove(j);
                    j--;
                }
            }

            if (duplicateGroup.size() > 1) {
                duplicateGroups.add(duplicateGroup);
            }
        }

        return duplicateGroups;
    }
    

    private void storePaths(String[] paths) {
        for (String path: paths) {
            List<FileEntry> entries = extractFileEntries(path);

            for(FileEntry fileEntry: entries) {
                int high = Math.min(fileEntry.content.length(), KEY_LEN_THRESHOLD);
                String key = fileEntry.content.substring(0, high);
                fileEntries.putIfAbsent(key, new ArrayList<>());
                List<FileEntry> values = fileEntries.get(key);
                values.add(fileEntry);
            }
        }
    }

    private List<FileEntry> extractFileEntries(String path) {
        List<FileEntry> fileEntries = new ArrayList<>();
        int dirEndPos = path.indexOf(" ");
        
        String dirName = path.substring(0, dirEndPos);
        String filePathsInfo = path.substring(dirEndPos + 1);

        String[] filePathInfos = filePathsInfo.split(" ");

        for (String filePathInfo: filePathInfos) {
            FileEntry fileEntry = new FileEntry();

            int openParen = filePathInfo.lastIndexOf("(");
            int closeParen = filePathInfo.lastIndexOf(")");

            fileEntry.path = dirName + "/" + filePathInfo.substring(0, openParen);
            fileEntry.content = filePathInfo.substring(openParen + 1, closeParen);

            fileEntries.add(fileEntry);
        } 

        return fileEntries;
    }

    public static void main(String... args) {
        List<List<String>> duplicates = new Solution().findDuplicate(new String[]{
            "root/a 1.txt(abcd) 2.txt(efgh)","root/c 3.txt(abcd)","root/c/d 4.txt(efgh)","root 4.txt(efgh)" });

        for(List<String> dupl: duplicates) {
            System.out.println(dupl);
        }    

    }

    static class FileEntry {
        public String content;
        
        public  String path;

        @Override
        public String toString() {
            return path + "(" + content + ")";
        }
    }
}