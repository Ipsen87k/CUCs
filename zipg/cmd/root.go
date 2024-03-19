/*
Copyright © 2024 Ipsen87k <EMAIL ADDRESS>
*/
package cmd

import (
	"os"

	"github.com/spf13/cobra"

	"github.com/Ipsen87k/CUCs/zipg/core"
)

var (
	isUnZip        bool
	outputFilePath string
	srcFilePath    []string
)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "zipg",
	Short: "A brief description of your application",
	Long: `A longer description that spans multiple lines and likely contains
examples and usage of using your application. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		if len(args) == 0 && args[0] == "" {
			currentDir, err := os.Getwd()
			cobra.CheckErr(err)
			args = []string{currentDir}
		}
		if isUnZip {
			if outputFilePath != "" {
				core.UnZip(args[0], outputFilePath)
			} else {
				core.UnZip(args[0], "")
			}
			return
		}

		if outputFilePath == "" {
			outputFilePath = "output"
		}
		outputFilePath += ".zip"
		core.ConvertToZip(outputFilePath, args)

	},
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {

	rootCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
	rootCmd.Flags().BoolVarP(&isUnZip, "unzip", "u", false, "zipg -u [compressed directory or files]")
	rootCmd.Flags().StringVarP(&outputFilePath, "output", "o", "", "zip -o [output filepath]")
	rootCmd.Flags().StringSliceVarP(&srcFilePath, "path", "p", make([]string, 10), "zip -p [dirctory or file]")
}
